use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
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
        embed_boot_splash();
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

/// Rasterize `assets/splash/boot.{png,jpg,jpeg}` for Pico DVI (640×465 RGB + status bar).
fn embed_boot_splash() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let splash_dir = manifest_dir.join("assets/splash");
    let source = ["boot.png", "boot.jpg", "boot.jpeg"]
        .into_iter()
        .map(|name| splash_dir.join(name))
        .find(|path| path.exists());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest = out_dir.join("boot_splash_embedded.rs");

    let Some(source) = source else {
        fs::write(
            &dest,
            "pub const SPLASH_W: u32 = 0;\npub const SPLASH_H: u32 = 0;\npub const SPLASH_RGB: &[u8] = &[];\n",
        )
        .expect("write empty boot splash");
        return;
    };

    let img = image::open(&source).unwrap_or_else(|e| panic!("{}: {e}", source.display()));
    const W: u32 = 640;
    const H: u32 = 465; // DISPLAY_HEIGHT (480) − FONT_HEIGHT (15)
    let rgb = image::imageops::resize(
        &img.to_rgb8(),
        W,
        H,
        image::imageops::FilterType::Triangle,
    )
    .into_raw();

    let mut rust = String::from("pub const SPLASH_W: u32 = 640;\npub const SPLASH_H: u32 = 465;\npub const SPLASH_RGB: &[u8] = &[\n");
    for (i, byte) in rgb.iter().enumerate() {
        if i % 16 == 0 {
            rust.push_str("    ");
        }
        write!(rust, "0x{byte:02x}, ").unwrap();
        if i % 16 == 15 {
            rust.push('\n');
        }
    }
    if rgb.len() % 16 != 0 {
        rust.push('\n');
    }
    rust.push_str("];\n");
    fs::write(&dest, rust).expect("write boot_splash_embedded.rs");
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
