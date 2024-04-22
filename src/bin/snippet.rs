//! scratchpad for testing code snippets
fn main() {
    let mut buffer: String = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    let _n: u64 = buffer.trim().parse().unwrap();
    // buffer.clear();
    // std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    // let sum: u64 = n * (n + 1) / 2;
    // let partial_sum: u64 = buffer
    //     .split_whitespace()
    //     .map(|x| x.parse::<u64>().unwrap())
    //     .sum();
    // println!("{}", sum - partial_sum);
}
