fn main() {
    let mut binaries = ["repetitions", "missing_number", "weird_algorithm"];
    binaries.sort();
    println!("\nCSES Rust Solutions");
    println!("===================");
    println!("Use the `--bin` option to specify a binary");
    println!("Available binaries:");
    for binary in binaries.iter() {
        println!("  {}", binary);
    }
    println!();
}
