//! increasing array problem from CSES problem set (https://cses.fi/problemset/task/1094)

fn main() {
    let mut buffer: String = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    buffer.clear();
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
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
