#[allow(dead_code)]
mod intrinsics;

use cc::Build;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    generate()?;

    Ok(())
}

/// Generates object for LLVM IR integer intrinsics. This enables the crate to
/// use compiler generated `U256` operations (such as addition, multiplication)
/// instead of native Rust implementation.
///
/// Setting the environment variable `RUST_ETHNUM_GENERATE_INTRINSICS=1` enables
/// generated intrinsics.
///
/// Note that generating intrinsics requires a Clang toolchain to compile LLVM
/// IR, but has some advantages. Specifically, it detects and enables ThinLTO
/// for the compiled LLVM IR, which allows the linker to perform link-time
/// optimizations such as inlining some of the intrinsics (such as `add*`) which
/// has some **REAL** performance benefits.
///
/// Returns `true` if intrinsics were generated, `false` otherwise.
fn generate() -> Result<bool> {
    const VAR: &str = "RUST_ETHNUM_GENERATE_INTRINSICS";

    println!("cargo:rerun-if-env-changed={}", VAR);
    if !matches!(env::var(VAR), Ok(var) if var == "1") {
        return Ok(false);
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let template = {
        let path = out_dir.join("template.ll");
        Command::new(env::var("RUSTC")?)
            .arg("build/intrinsics.rs")
            .arg("-O")
            .args(&["--crate-type", "lib"])
            .args(&["--emit", "llvm-ir"])
            .args(&["--target", &env::var("TARGET")?])
            .arg("-o")
            .arg(&path)
            .status()?;
        fs::read_to_string(path)?
    };

    let intrinsics_ir_path = {
        let source = template
            .replace("i128", "i256")
            .replace(" 127", " 255")
            .replace("dereferenceable(16)", "dereferenceable(32)");
        let path = out_dir.join("intrinsics.ll");
        fs::write(&path, source)?;
        path
    };

    let mut build = Build::new();
    build
        .compiler("clang")
        .file(intrinsics_ir_path)
        .opt_level(3);

    let linker_plugin_lto =
        matches!(env::var("RUSTFLAGS"), Ok(flags) if flags.contains("-Clinker-plugin-lto"));
    if linker_plugin_lto {
        build.flag("-flto=thin");
    }

    build.try_compile("num")?;

    println!("cargo:rustc-cfg=generate_intrinsics");
    Ok(true)
}
