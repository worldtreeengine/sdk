use std::process::Command;
use which::which;

fn main() {
    println!("cargo::rerun-if-changed=../engine");

    let status = Command::new(which("pnpm").unwrap())
        .arg("--dir").arg("../engine/standalone/browser")
        .arg("run").arg("build-dev")
        .status().expect("Failed to execute bundle build");

    assert!(status.success());
}
