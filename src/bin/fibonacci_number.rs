use std::{
    io::{BufReader, Read},
    ops::Mul,
    str::SplitAsciiWhitespace,
};

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut buffer = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: u64 = get_token(&mut tokens);
    println!("{}", fib(n));
}

fn fib(n: u64) -> u64 {
    let mut t = Matrix::new(0, 1, 1, 1);
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    t = t.pow(n - 1);
    (t.rows[0][0] + t.rows[0][1]) % MOD
}

#[derive(Clone, Copy, Debug)]
struct Matrix {
    rows: [[u64; 2]; 2],
}

impl Matrix {
    fn new(a: u64, c: u64, b: u64, d: u64) -> Self {
        Matrix {
            rows: [[a, c], [b, d]],
        }
    }

    fn pow(self, n: u64) -> Self {
        if n == 1 {
            return self;
        }
        if n % 2 == 1 {
            return self * self.pow(n - 1);
        }
        let res = self.pow(n / 2);
        res * res
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut res = Matrix::new(0, 0, 0, 0);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    res.rows[i][j] = (res.rows[i][j] + self.rows[i][k] * rhs.rows[k][j]) % MOD;
                }
            }
        }
        res
    }
}

fn get_token<T: std::str::FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    if let Some(token) = tokens.next() {
        match token.parse::<T>() {
            Ok(r) => r,
            Err(_) => panic!("PARSE ERROR"),
        }
    } else {
        panic!("EXPECTED SOME, GOT NONE");
    }
}

fn load_tokens(buffer: &mut String) -> SplitAsciiWhitespace {
    let mut reader = BufReader::new(std::io::stdin());
    reader.read_to_string(buffer).expect("READ ERROR");
    buffer.split_ascii_whitespace()
}
