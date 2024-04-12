fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let n: u64 = buffer.trim().parse().unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let sum = n * (n + 1) / 2;
    let partial_sum: u64 = buffer
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .sum();
    println!("{}", sum - partial_sum);
}
