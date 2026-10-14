use std::process::Command;

fn main() {
    let status = Command::new("python3")
        .arg("gen/bitinterleaving.py")
        .arg("--output-dir")
        .arg("src")
        .arg("--lang")
        .arg("rust")
        .status()
        .expect("failed to run python");
    assert!(status.success(), "python script failed");

    // re-run only when the script or its inputs change
    println!("cargo:rerun-if-changed=gen/bitinterleaving.py");
}
