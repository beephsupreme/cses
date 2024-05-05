use std::fmt::Write;

fn main() {
    let mut tokens = Scanner::default();
    let n: u8 = tokens.next();
    let mut v: Vec<(u8, u8)> = Vec::new();
    hanoi(n, 1, 3, &mut v);
    let mut buffer = String::new();
    writeln!(buffer, "{}", v.len()).unwrap();
    v.iter()
        .for_each(|(a, b)| writeln!(buffer, "{} {}", a, b).unwrap());
    print!("{}", buffer)
}

fn hanoi(n: u8, a: u8, b: u8, v: &mut Vec<(u8, u8)>) {
    if n == 1 {
        v.push((a, b));
    } else {
        let c: u8 = 6 - (a + b);
        hanoi(n - 1, a, c, v);
        v.push((a, b));
        hanoi(n - 1, c, b, v)
    }
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
