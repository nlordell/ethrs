#[allow(dead_code)]
mod intrinsics;

use cc::Build;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let build = match (is_linker_plugin_lto_enabled(), pregenerated()?) {
        (false, Some(build)) => build,
        (true, _) | (_, None) => {
            // NOTE: if linker plugin LTO is enabled, always generate and
            // compile the LLVM IR so that we can enable ThinLTO with Clang.
            generate()?
        }
    };

    build.try_compile("num")?;

    Ok(())
}

/// Looks for a pre-generated intrinsics source.
fn pregenerated() -> Result<Option<Build>> {
    let target = env::var("TARGET")?;
    for file in fs::read_dir("src/arch")? {
        let file = file?;
        if target.starts_with(&*file.file_name().to_string_lossy()) {
            let source = file.path().join("intrinsics.s");
            let mut build = Build::new();
            build.file(source);

            return Ok(Some(build));
        }
    }

    Ok(None)
}

/// Generates object for LLVM IR integer intrinsics. This enables the crate to
/// use compiler generated `u256` operations (such as addition, multiplication)
/// instead of implementing by hand.
///
/// Note that generating intrinsics requires a Clang toolchain to compile LLVM
/// IR, but has some advantages. Specifically, it detects and enables ThinLTO
/// for the compiled LLVM IR, which allows the linker to perform link-time
/// optimizations such as inlining some of the intrinsics (such as `add*`) which
/// has some REAL performance benefits.
fn generate() -> Result<Build> {
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

    if is_linker_plugin_lto_enabled() {
        build.flag("-flto=thin");
    }

    Ok(build)
}

/// Returns true if Rust is configured for linker plugin LTO.
fn is_linker_plugin_lto_enabled() -> bool {
    matches!(env::var("RUSTFLAGS"), Ok(flags) if flags.contains("-Clinker-plugin-lto"))
}
