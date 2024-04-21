//! repetitions problem from CSES problem set (https://cses.fi/problemset/task/1069)

pub fn main() {
    let mut buffer: String = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    let buffer: String = buffer.trim().to_string();
    let mut streak: u64 = 1;
    let mut longest: u64 = 1;
    let mut prev: u8 = b'@';
    buffer.as_bytes().iter().for_each(|&ch| match ch == prev {
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
