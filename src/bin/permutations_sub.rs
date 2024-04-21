//! permutations problem from CSES problem set (https://cses.fi/problemset/task/1070)

fn main() {
    let mut buffer: String = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    let mut tokens = buffer.split_ascii_whitespace();
    let n: u64 = tokens.next().unwrap().parse().unwrap();
    match n {
        1 => println!("1"),
        0 | 2 | 3 => println!("NO SOLUTION"),
        _ => {
            let half = n / 2;
            for i in 0..half {
                println!("{} ", 2 * i + 2);
            }
            for i in 0..n - half {
                println!("{} ", 2 * i + 1);
            }
        }
    };
}
