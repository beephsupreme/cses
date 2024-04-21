//! number spiral problem from CSES problem set (https://cses.fi/problemset/task/1071)

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut tokens = buffer.split_ascii_whitespace();
    let n: u64 = tokens.next().unwrap().parse().unwrap();
    for _ in 0..n {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut tokens = buffer.split_ascii_whitespace();
        let a: u64 = tokens.next().unwrap().parse().unwrap();
        let b: u64 = tokens.next().unwrap().parse().unwrap();
        let max = u64::max(a, b);
        let d = 1 + max * (max - 1);
        if max & 1 == 0 {
           println!("{}", d + a - b);
        } else {
             println!("{}", d + b - a);
        }
    }
}

