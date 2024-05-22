use std::fmt::Write;

fn main() {
    let mut tokens = Scanner::default();
    let mut output: String = String::new();
    let m: i64 = tokens.next();
    for _ in 0..m {
        let mut n: i64 = tokens.next();
        let t = 10_i64;
        let mut p = 9_i64;
        let mut c = 1_i64;
        loop {
            if n - p <= 0 {
                break;
            }
            n -= p;
            c += 1;
            p = 9 * t.pow(c as u32 - 1) * c;
        }
        n -= 1;
        let result: i64 = t.pow(c as u32 - 1) + n / c;
        let s = result.to_string().chars().nth((n % c) as usize).unwrap();
        writeln!(output, "{}", s).unwrap();
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
