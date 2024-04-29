use std::fs;

fn main() {
    let paths = fs::read_dir("./src/bin").expect("FILE READ ERROR");
    let mut filenames: Vec<String> = Vec::new();
    for p in paths {
        let p = p.expect("PATH ERROR");
        if !p.path().is_file() {
            continue;
        }
        let file = p.path().display().to_string();
        if !file.contains("cses") && file.contains(".rs") {
            let filename = file
                .strip_prefix("./src/bin/")
                .unwrap()
                .strip_suffix(".rs")
                .unwrap();
            filenames.push(filename.to_string());
        }
    }
    filenames.sort();
    println!("\nCSES Rust Solutions\n===================");
    println!("Use the `--bin` option to specify a binary");
    println!("Available binaries:");
    for filename in filenames {
        println!("  {}", filename);
    }
    println!();
}
