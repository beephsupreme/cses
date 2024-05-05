// Problem: Weird Algorithm
use std::fmt::Write;

fn main() {
    let mut scan = Scanner::default();
    let mut n: u64 = scan.next();
    let mut buffer = String::new();
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

#[derive(Default)]
struct Scanner {
    buffer: std::collections::VecDeque<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                return token.parse().ok().expect("PARSE ERROR");
            }
            let mut input = String::new();
            let mut reader = std::io::BufReader::new(std::io::stdin());
            std::io::Read::read_to_string(&mut reader, &mut input).expect("READ ERROR");
            self.buffer = input.split_ascii_whitespace().map(String::from).collect();
        }
    }
}
