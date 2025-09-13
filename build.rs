use std::process::Command;

fn main() {
    // Get short git hash (use "rev-parse HEAD" for full hash)
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to execute git");

    let git_hash = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 in git hash")
        .trim()
        .to_string();

    // Set environment variable accessible during compile
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Tell Cargo to rerun build script if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
