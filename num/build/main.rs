mod intrinsics;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let intrinsics = intrinsics::source()?;
    cc::Build::new().file(intrinsics).compile("num");

    Ok(())
}
