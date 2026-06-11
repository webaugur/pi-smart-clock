use std::env;
use std::process::Command;

fn main() {
    // Git hash for status bar and About page (desktop build only now)
    let short = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let full = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={}", short);
    println!("cargo:rustc-env=GIT_FULL_HASH={}", full);

    let data_dir = env::var("PI_DATA_DIR")
        .or_else(|_| env::var("CARGO_MANIFEST_DIR"))
        .expect("PI_DATA_DIR or CARGO_MANIFEST_DIR");
    println!("cargo:rerun-if-env-changed=PI_DATA_DIR");
    println!("cargo:rustc-env=PI_DATA_DIR={data_dir}");

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
    for name in ["boot.png", "boot.jpg", "boot.jpeg"] {
        println!("cargo:rerun-if-changed=assets/splash/{name}");
    }
}
