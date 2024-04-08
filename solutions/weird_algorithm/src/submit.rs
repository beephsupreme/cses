use std::io::{BufRead, BufReader};
use std::fmt::Write;

fn main() {
    let mut buffer: String = String::new();
    let mut reader = BufReader::new(std::io::stdin());
    reader.read_line(&mut buffer).unwrap();
    let mut n: u64 = buffer.trim().parse().unwrap();
    buffer.clear();
    loop {
        write!(buffer, "{} ", n).unwrap();
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    println!("{}", buffer.trim());
}