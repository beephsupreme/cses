fn main() {
    let mut tokens = Scanner::default();
    let n: i64 = tokens.next();
    let mut v: Vec<i64> = Vec::new();
    for _ in 0..n {
        v.push(tokens.next::<i64>());
    }
    v.sort();
    let n: i64 = v.iter().sum();
    println!("{}", solve(&v, v[0], 0, 1, n, n >> 1));
}

fn solve(v: &[i64], a: i64, b: i64, i: usize, n: i64, m: i64) -> i64 {
    match m {
        q if a >= q => return i64::abs(n - (a << 1)),
        q if b >= q => return i64::abs(n - (b << 1)),
        _ => {
            return std::cmp::min(
                solve(v, a + v[i], b, i + 1, n, m),
                solve(v, a, b + v[i], i + 1, n, m),
            )
        }
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
