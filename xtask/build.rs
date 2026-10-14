use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let katwalk_manifest = manifest_dir.join("../../katwalk/Cargo.toml");
    println!("cargo:rerun-if-changed={}", katwalk_manifest.display());

    let manifest = fs::read_to_string(&katwalk_manifest)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", katwalk_manifest.display()));
    let manifest: toml::Value = manifest
        .parse()
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", katwalk_manifest.display()));
    let version = manifest["package"]["version"]
        .as_str()
        .expect("katwalk Cargo.toml has no package version");
    println!("cargo:rustc-env=KATWALK_VERSION={version}");

    let project_manifest = manifest_dir.join("../Cargo.toml");
    println!("cargo:rerun-if-changed={}", project_manifest.display());
    let manifest = fs::read_to_string(&project_manifest)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", project_manifest.display()));
    let manifest: toml::Value = manifest
        .parse()
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", project_manifest.display()));
    let revision = manifest["package"]["metadata"]["kat"]["vector_revision"]
        .as_str()
        .expect("Cargo.toml has no KAT vector revision");
    let repository = manifest["package"]["metadata"]["kat"]["vector_repository"]
        .as_str()
        .expect("Cargo.toml has no KAT vector repository");
    println!("cargo:rustc-env=VECTOR_REPOSITORY={repository}");
    println!("cargo:rustc-env=VECTOR_REVISION={revision}");
}
