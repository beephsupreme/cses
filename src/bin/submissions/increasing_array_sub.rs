fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<u64> = buffer
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut previous = v[0];
    let mut current: u64 = 0;
    for e in v {
        if e < previous {
            current += previous - e;
        } else {
            previous = e;
        }
    }
    println!("{}", current);
}
