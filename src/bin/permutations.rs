use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    let mut tokens = Scanner::default();
    let n: u64 = tokens.next();
    buffer.clear();
    match n {
        1 => println!("1"),
        0 | 2 | 3 => write!(buffer, "NO SOLUTION").unwrap(),
        _ => {
            let half = n / 2;
            for i in 0..half {
                write!(buffer, "{} ", 2 * i + 2).unwrap();
            }
            for i in 0..n - half {
                write!(buffer, "{} ", 2 * i + 1).unwrap();
            }
        }
    };
    println!("{buffer}");
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
