use std::fmt::Write;

fn main() {
    let mut tokens = Scanner::default();
    let n: usize = tokens.next();
    let mut buffer = String::new();
    for i in 0..(1 << n) {
        let gray_code = i ^ (i >> 1);
        writeln!(buffer, "{:0width$b}", gray_code, width = n).unwrap();
    }
    println!("{}", buffer);
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
