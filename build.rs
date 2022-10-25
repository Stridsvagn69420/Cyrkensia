use std::process::Command;
use std::env;
use chrono::Utc;

fn version(name: &str) -> String {
    let output = Command::new(name).arg("--version").output().unwrap();
    let raw = String::from_utf8(output.stdout).unwrap();
    raw.replace(&format!("{} ", name), "")
}

fn main() {
    // ----- Target Triple -----
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

    // ----- Rust Version -----
    println!("cargo:rustc-env=CARGO_VERSION={}", version("cargo"));
    println!("cargo:rustc-env=RUSTC_VERSION={}", version("rustc"));

    // ----- Compile Date -----
    println!("cargo:rustc-env=COMPILE_DATE={}", Utc::now().format("%Y-%m-%d"));
}