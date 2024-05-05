use std::fmt::Write;

fn main() {
    let mut buffer = String::new();
    let mut tokens = Scanner::default();
    let n: u64 = tokens.next();
    buffer.clear();

    let (mut a, mut b) = (Vec::new(), Vec::new());
    let mut sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        println!("NO");
        return;
    } else {
        sum /= 2;
    }
    for i in (1..=n).rev() {
        if i <= sum {
            sum -= i;
            a.push(i);
        } else {
            b.push(i)
        }
    }
    writeln!(buffer, "YES").unwrap();
    writeln!(buffer, "{}", a.len()).unwrap();
    a.iter().for_each(|e| write!(buffer, "{} ", e).unwrap());
    writeln!(buffer, "\n{}", b.len()).unwrap();
    b.iter().for_each(|e| write!(buffer, "{} ", e).unwrap());
    writeln!(buffer).unwrap();
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
