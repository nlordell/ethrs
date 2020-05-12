#[allow(dead_code)]
mod template;

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn source() -> Result<PathBuf, Box<dyn Error>> {
    let path = match find()? {
        Some(found) => found,
        None => generate()?,
    };

    Ok(path)
}

fn find() -> Result<Option<PathBuf>, Box<dyn Error>> {
    let target = env::var("TARGET")?;

    for file in fs::read_dir("src/arch")? {
        let file = file?;
        if target.starts_with(&*file.file_name().to_string_lossy()) {
            let source = file.path().join("intrinsics.s");
            return Ok(Some(source));
        }
    }

    Ok(None)
}

fn generate() -> Result<PathBuf, Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let template = {
        let path = out_dir.join("template.ll");
        Command::new(env::var("RUSTC")?)
            .arg("build/intrinsics/template.rs")
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

    let intrinsics_path = {
        let path = out_dir.join("intrinsics.s");
        Command::new("llc")
            .arg(&intrinsics_ir_path)
            .arg(format!("-mtriple={}", env::var("TARGET")?))
            .arg("-o")
            .arg(&path)
            .status()?;
        path
    };

    Ok(intrinsics_path)
}