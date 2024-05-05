use std::fmt::Write;

fn main() {
    let mut tokens = Scanner::default();
    let input: String = tokens.next();
    let mut freq: [u64; 26] = [0; 26];
    let mut middle: Option<u8> = None;

    for c in input.chars() {
        freq[(c as u8 - b'A') as usize] += 1;
    }

    for (i, f) in freq.iter().enumerate() {
        if f % 2 == 1 {
            if middle.is_some() {
                println!("NO SOLUTION");
                return;
            }
            middle = Some(i as u8 + b'A');
        }
    }

    let mut front = String::new();
    for (i, f) in freq.iter().enumerate() {
        for _ in 0..f / 2 {
            front.push((i as u8 + b'A') as char);
        }
    }

    let back: String = front.chars().rev().collect();
    let mut output = String::new();
    write!(output, "{}", front).unwrap();

    if middle.is_some() {
        write!(output, "{}", middle.unwrap() as char).unwrap();
    }

    write!(output, "{}", back).unwrap();
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
