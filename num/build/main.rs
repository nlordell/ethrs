mod intrinsics;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let intrinsics = intrinsics::source()?;
    cc::Build::new()
        .file(intrinsics)
        .compile("num");

    Ok(())
}
