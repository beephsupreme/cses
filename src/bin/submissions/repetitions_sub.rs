pub fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let buffer: String = buffer.trim().to_string();
    let mut streak = 1u64;
    let mut longest = 1u64;
    let mut prev = b'@';
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
