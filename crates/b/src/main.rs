use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    // dotenv() uses current_dir
    println!("current dir: {}", env::current_dir()?.to_string_lossy());
    println!("{}", env::var("FOO")?);

    // this is the crate dir
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    println!("cargo manifest dir: {manifest_dir}");
    dotenvy::from_path_override(format!("{manifest_dir}/.env"))?;
    println!("{}", env::var("FOO")?);
    Ok(())
}
