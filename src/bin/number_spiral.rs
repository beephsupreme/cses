use std::fmt::Write;

fn main() {
    let mut output = String::new();
    let mut tokens = Scanner::default();
    let n: u64 = tokens.next();
    for _ in 0..n {
        let a: u64 = tokens.next();
        let b: u64 = tokens.next();
        let max = u64::max(a, b);
        let d = 1 + max * (max - 1);
        if max & 1 == 0 {
            writeln!(output, "{}", d + a - b).unwrap();
        } else {
            writeln!(output, "{}", d + b - a).unwrap();
        }
    }
    println!("{}", output);
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
