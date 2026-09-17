use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dll = manifest_dir.join("interception.dll");
    let lib = manifest_dir.join("interception.lib");

    if !dll.exists() {
        panic!("Missing {}", dll.display());
    }

    if !lib.exists() {
        panic!("Missing {}", lib.display());
    }

    // Re-run build script when native files change.
    println!("cargo:rerun-if-changed={}", dll.display());
    println!("cargo:rerun-if-changed={}", lib.display());

    // Tell rustc where interception.lib is.
    println!("cargo:rustc-link-search=native={}", manifest_dir.display());

    // Link interception.lib.
    println!("cargo:rustc-link-lib=dylib=interception");

    // OUT_DIR:
    // target/debug/build/<package>/out

    // OUT_DIR: target/<profile>/build/<pkg>-<hash>/out
    // out -> <pkg>-<hash> -> build -> <profile>
    let profile_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // In a normal build:
    // target/debug/...  or target/release/...

    // OUT_DIR is:
    // target/debug/build/...
    // so profile_dir becomes target/debug.
    let destination = profile_dir.join("interception.dll");

    fs::copy(&dll, &destination).unwrap_or_else(|error| {
        panic!(
            "Failed to copy {} -> {}: {}",
            dll.display(),
            destination.display(),
            error
        )
    });

    println!(
        "cargo:warning=Copied interception.dll to {}",
        destination.display()
    );
}
