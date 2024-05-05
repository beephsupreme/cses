fn main() {
    let mut tokens = Scanner::default();
    let input: String = tokens.next();
    let mut streak: u64 = 1;
    let mut longest: u64 = 1;
    let mut prev: u8 = b'@';
    input.as_bytes().iter().for_each(|&ch| match ch == prev {
        true => {
            streak += 1;
            longest = std::cmp::max(longest, streak);
        }
        false => {
            streak = 1;
            prev = ch;
        }
    });
    println!("{longest}");
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
