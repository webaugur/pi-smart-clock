use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let linux = env::var_os("CARGO_FEATURE_LINUX_FULL").is_some();
    let pico = env::var_os("CARGO_FEATURE_PICO_DVI").is_some();
    if linux && pico {
        panic!(
            "features `linux-full` and `pico-dvi` are mutually exclusive.\n\
             Linux:  cargo run --features linux-full\n\
             Pico:   cargo pico   (see .cargo/config.toml)"
        );
    }

    if pico {
        check_embedded_toolchain();
    }
    // Short hash for status bar
    let short = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Full hash for About page
    let full = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={}", short);
    println!("cargo:rustc-env=GIT_FULL_HASH={}", full);

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
}

fn check_embedded_toolchain() {
    let target = env::var("TARGET").unwrap_or_default();
    if target != "thumbv6m-none-eabi" {
        return;
    }

    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let rustc_path = which(&rustc);

    if rustc_path.as_deref() == Some("/usr/bin/rustc") {
        panic!(
            "Pico builds require rustup's rustc, not apt's /usr/bin/rustc.\n\
             Fix PATH so ~/.cargo/bin comes first:\n\
               export PATH=\"$HOME/.cargo/bin:$PATH\"\n\
             Then run:  ./scripts/setup-embedded.sh\n\
             Optional: sudo apt remove rustc cargo   (removes the conflict)"
        );
    }

    let ok = Command::new(&rustc)
        .args(["--print", "sysroot", "--target", "thumbv6m-none-eabi"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !ok {
        panic!(
            "thumbv6m-none-eabi target is not installed for the active rustc.\n\
             Run:  ./scripts/setup-embedded.sh\n\
             Or:   rustup target add thumbv6m-none-eabi"
        );
    }
}

fn which(cmd: &str) -> Option<String> {
    Command::new("which")
        .arg(cmd)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty() && Path::new(s).exists())
}
