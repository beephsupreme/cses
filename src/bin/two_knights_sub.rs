//! two knights problem from CSES problem set (https://cses.fi/problemset/task/1072)
pub fn main() {
    let mut buffer: String = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    let n: i64 = buffer.trim().parse().unwrap();
    buffer.clear();
    for i in 1..=n {
        println!("{}", (i.pow(4) - 9 * i.pow(2) + 24 * i - 16) / 2);
    }
}
