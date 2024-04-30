use std::{
    io::{BufReader, Read},
    ops::Mul,
    str::SplitAsciiWhitespace,
};

const    MOD: u64 = 1_000_000_007;
const      K: u64 = 2;

fn main() {
    let a = Matrix::new(0, 1, 1, 1);
    
    
    let b = a * a * a;
    let c = a.pow(3);
    println!("{:?}", b);
    println!("{:?}", c);

    // let mut buffer = String::new();
    // let mut tokens = load_tokens(&mut buffer);
    // let mut n: u64 = get_token(&mut tokens);
    // n = n % PISANO;
    // let mut p = 0;
    // let mut c = 1;
    // let mut t;
    // if n == 0 || n == 1 {
    //     println!("{}", n);
    //   return
    // }
    // for _ in 0..n-1 {
    //     t = c;
    //     c = (p + c) % MOD;
    //     p = t;
    // }
    // println!("{}", c % MOD);
}

// // returns the N-th term of Fibonacci sequence
// int fib(int N)
// {
//     // create vector F1
//     vector<ll> F1(K+1);
//     F1[1] = 1;
//     F1[2] = 1;

//     // create matrix T
//     matrix T(K+1, vector<ll>(K+1));
//     T[1][1] = 0, T[1][2] = 1;
//     T[2][1] = 1, T[2][2] = 1;

//     // raise T to the (N-1)th power
//     if (N == 1)
//         return 1;
//     T = pow(T, N-1);

//     // the answer is the first row of T . F1
//     ll res = 0;
//     REP(i, K)
//         res = (res + T[1][i] * F1[i]) % MOD;
//     return res;
// }


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

impl Matrix {
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
