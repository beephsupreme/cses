//! number spiral problem from CSES problem set (https://cses.fi/problemset/task/1068)

fn main() {
    let mut buffer: String = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    let mut n: u64 = buffer.trim().parse().unwrap();
    loop {
        print!("{} ", n);
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    println!("{}", buffer.trim());
}
