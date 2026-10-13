use std::process::Command;

fn run(script: &str) {
    let status = Command::new("python3")
        .arg(script)
        .arg("--output-dir")
        .arg("src")
        .arg("--lang")
        .arg("rust")
        .status()
        .expect("failed to run python");
    assert!(status.success(), "{script} failed");
    println!("cargo:rerun-if-changed={script}");
}

fn main() {
    run("gen/bitinterleaving.py");
    run("gen/permutation_rv32.py");
}
