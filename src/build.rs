//! This class is called during build. Can be used to handle versioning and other things uwu

use std::process::Command;
fn main() {
    // note: add error checking yourself.
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println("cargo:rustc-env=GIT_HASH={}", git_hash);
    env!("GIT_HASH", &git_hash);
}