fn main() {
    let mut tokens = Scanner::default();
    let n: u64 = tokens.next();
    let mut previous: u64 = tokens.next();
    let mut current: u64 = 0;
    for _ in 1..n {
        let e: u64 = tokens.next();
        if e < previous {
            current += previous - e;
        } else {
            previous = e;
        }
    }
    println!("{}", current);
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
