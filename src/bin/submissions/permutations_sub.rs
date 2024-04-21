use std::fmt::Write;
use std::io::{BufRead, BufReader};
use std::ptr::write;

fn main() {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin.lock());
    let n: u64 = reader.lines().next().unwrap().unwrap().parse().unwrap();
    let mut output = String::new();
    match n {
        1 => Some(vec![1]),
        0 | 2 | 3 => println!("NO SOLUTION"),
        _ => {
            let mut v: Vec<u64> = Vec::with_capacity(n as usize);
            let half = n / 2;
            for i in 0..half {
                write!(output, "{} ", 2 * i + 2).unwrap();
            }
            for i in 0..n - half {
                write!(output, "{} ", 2 * i + 1).unwrap();
            }
        }
    };
    println!("{}", output);
}
