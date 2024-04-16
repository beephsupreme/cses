use cses::prelude::*;
use cses::utils::get_binary_filenames;

fn main() -> Result<()> {
    let filenames = get_binary_filenames()?;
    println!("\nCSES Rust Solutions\n===================");
    println!("Use the `--bin` option to specify a binary");
    println!("Available binaries:");
    for filename in filenames {
        println!("  {}", filename);
    }
    println!();
    Ok(())
}
