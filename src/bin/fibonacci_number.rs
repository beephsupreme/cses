const MOD: u64 = 1_000_000_007;

fn main() {
    let mut tokens = Scanner::default();
    let n: u64 = tokens.next();
    println!("{}", fib(n));
}

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let t = Matrix(0, 1, 1, 1);
            let t = pow(t, n - 1);
            t.3 % MOD
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Matrix(u64, u64, u64, u64);

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Matrix(
            (self.0 * rhs.0 + self.1 * rhs.2) % MOD,
            (self.0 * rhs.1 + self.1 * rhs.3) % MOD,
            (self.2 * rhs.0 + self.3 * rhs.2) % MOD,
            (self.2 * rhs.1 + self.3 * rhs.3) % MOD,
        )
    }
}

fn pow(mut a: Matrix, mut n: u64) -> Matrix {
    let mut result = Matrix(1, 0, 0, 1);
    while n > 0 {
        if n % 2 == 1 {
            result = result * a;
        }
        a = a * a;
        n /= 2;
    }
    result
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
