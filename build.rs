use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let frugalos_dir: std::ffi::OsString = env::var_os("FRUGALOS_DIR").unwrap();
    let frugalos_dir: String = frugalos_dir.into_string().unwrap();

    println!("cargo:rustc-link-search={}/lib", frugalos_dir);
}
