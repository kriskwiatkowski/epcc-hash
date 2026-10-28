use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

const VECTOR_REPOSITORY: &str = match option_env!("VECTOR_REPOSITORY") {
    Some(value) => value,
    None => "https://github.com/kriskwiatkowski/crypto-test-vectors.git",
};
const VECTOR_REVISION: &str = match option_env!("VECTOR_REVISION") {
    Some(value) => value,
    None => "159c3ef0068184ff85836b36db8d2e4f06d4d1ac",
};

fn main() {
    match env::args().nth(1).as_deref() {
        Some("kat") => run_kats(),
        Some("docs") => run_docs(env::args().nth(2).as_deref()),
        _ => Err("usage: cargo xtask kat | cargo xtask docs [--open|--eisvogel]".into()),
    }
    .unwrap_or_else(|error| {
        eprintln!("xtask: {error}");
        std::process::exit(1);
    });
}
fn run_kats() -> Result<(), Box<dyn std::error::Error>> {
    let root = workspace_root()?;
    let target = root.join("target");
    let vectors = target.join("crypto-test-vectors");

    checkout_vectors(&vectors)?;

    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let mut build = Command::new("cargo");
    build.current_dir(&root).args([
        "build",
        "-p",
        "epcc-keccak-kat",
        "--bin",
        "fips202_wrapper",
    ]);
    if profile == "release" {
        build.arg("--release");
    }
    build.arg("--features").arg("katwalk-bin");
    run(&mut build)?;

    let wrapper = target.join(profile).join("fips202_wrapper");
    let fips_vectors = vectors.join("vectors/acvp/FIPS-202");
    run_kat(
        Path::new("katwalk"),
        &wrapper,
        &fips_vectors.join("SHA3-256"),
        &target.join("fips202_out.json"),
    )
    .map_err(|error| -> Box<dyn std::error::Error> {
        format!(
            "{error} (is katwalk installed and on PATH? see .github/workflows/ci.yml \
             for the pinned `cargo install katwalk` command)"
        )
        .into()
    })
}

fn run_docs(flag: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    match flag {
        Some("--eisvogel") => run_docs_eisvogel(),
        _ => run_docs_mdbook(flag),
    }
}

fn run_docs_mdbook(flag: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let root = workspace_root()?;
    let docs = root.join("docs");

    let mut build = Command::new("mdbook");
    build.current_dir(&root).arg("build").arg(&docs);
    if flag == Some("--open") {
        build.arg("--open");
    }
    run(&mut build).map_err(|error| {
        format!(
            "{error} (is mdbook installed? `cargo install mdbook`)"
        )
        .into()
    })
}

// Alternative PDF backend, tried out alongside mdbook-pdf: renders the same
// docs/src chapters through Pandoc + the Eisvogel LaTeX template instead of
// mdbook-pdf's headless-Chrome screenshot approach, giving a typeset-book
// look (title page, serif-free body font, proper page numbers). Needs
// `pandoc`, a LaTeX engine (xelatex) and the Eisvogel template installed at
// ~/.local/share/pandoc/templates/eisvogel.latex — see
// https://github.com/Wandmalfarbe/pandoc-latex-template. Eisvogel's default
// font (the `sourcesanspro` LaTeX package) isn't reliably present even with
// texlive-fonts-extra installed, so fonts are pinned explicitly to ones
// that ship with most Linux distros.
fn run_docs_eisvogel() -> Result<(), Box<dyn std::error::Error>> {
    let root = workspace_root()?;
    let docs = root.join("docs");
    let src = docs.join("src");

    let summary = fs::read_to_string(src.join("SUMMARY.md"))?;
    let chapters = chapter_files(&summary, &src);
    if chapters.is_empty() {
        return Err("no chapters found in docs/src/SUMMARY.md".into());
    }

    let title = book_title(&fs::read_to_string(docs.join("book.toml"))?)
        .unwrap_or_else(|| "Developer Guide".to_string());

    let out_dir = docs.join("book/pdf-eisvogel");
    fs::create_dir_all(&out_dir)?;
    let output = out_dir.join("output.pdf");

    let mut pandoc = Command::new("pandoc");
    pandoc
        .current_dir(&root)
        .args(&chapters)
        .args([
            "--from",
            "markdown",
            "--template",
            "eisvogel",
            "--syntax-highlighting=idiomatic",
            "--pdf-engine",
            "xelatex",
            "--toc",
        ])
        .arg("-V")
        .arg("titlepage=true")
        .arg("-V")
        .arg(format!("title={title}"))
        .arg("-V")
        .arg("toc-own-page=true")
        .arg("-V")
        .arg("mainfont=Liberation Sans")
        .arg("-V")
        .arg("monofont=Liberation Mono")
        .arg("-o")
        .arg(&output);

    run(&mut pandoc).map_err(|error| -> Box<dyn std::error::Error> {
        format!(
            "{error} (needs pandoc, xelatex and the eisvogel template at \
             ~/.local/share/pandoc/templates/eisvogel.latex)"
        )
        .into()
    })?;
    println!("PDF written to {}", output.display());
    Ok(())
}

// Pulls chapter paths out of SUMMARY.md's `[Title](file.md)` links, in
// order, ignoring nesting/indentation.
fn chapter_files(summary: &str, src: &Path) -> Vec<PathBuf> {
    summary
        .lines()
        .filter_map(|line| {
            let start = line.find("](")? + 2;
            let end = line[start..].find(')')? + start;
            Some(src.join(&line[start..end]))
        })
        .collect()
}

fn book_title(book_toml: &str) -> Option<String> {
    book_toml.lines().find_map(|line| {
        let rest = line.trim().strip_prefix("title")?.trim_start();
        let rest = rest.strip_prefix('=')?.trim();
        let rest = rest.strip_prefix('"')?;
        let end = rest.find('"')?;
        Some(rest[..end].to_string())
    })
}

fn checkout_vectors(vectors: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !vectors.exists() {
        run(Command::new("git")
            .args(["clone", "--no-checkout", VECTOR_REPOSITORY])
            .arg(vectors))?;
    }

    run(Command::new("git").args(["-C"]).arg(vectors).args([
        "fetch",
        "--depth",
        "1",
        "origin",
        VECTOR_REVISION,
    ]))?;
    run(Command::new("git").args(["-C"]).arg(vectors).args([
        "checkout",
        "--detach",
        VECTOR_REVISION,
    ]))
}

fn run_kat(
    katwalk: &Path,
    wrapper: &Path,
    test_directory: &Path,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    run(Command::new(katwalk).args([
        "--wrapper",
        wrapper.to_str().ok_or("wrapper path is not valid UTF-8")?,
        "--in",
        test_directory
            .join("prompt.json")
            .to_str()
            .ok_or("input path is not valid UTF-8")?,
        "--out",
        output.to_str().ok_or("output path is not valid UTF-8")?,
        "--expected",
        test_directory
            .join("expectedResults.json")
            .to_str()
            .ok_or("expected-results path is not valid UTF-8")?,
    ]))
}

fn workspace_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "xtask has no workspace parent".into())
}

fn run(command: &mut Command) -> Result<(), Box<dyn std::error::Error>> {
    let status = command.status()?;
    successful(status, command)
}

fn successful(
    status: ExitStatus,
    command: &Command,
) -> Result<(), Box<dyn std::error::Error>> {
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed ({status}): {command:?}").into())
    }
}
