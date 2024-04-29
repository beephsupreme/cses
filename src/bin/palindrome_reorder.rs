use std::fmt::Write;

fn main() {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    let mut freq: [u64; 26] = [0; 26];
    let mut middle: Option<u8> = None;

    for c in input.chars() {
        freq[(c as u8 - b'A') as usize] += 1;
    }

    for (i, f) in freq.iter().enumerate() {
        if f % 2 == 1 {
            if middle.is_some() {
                println!("NO SOLUTION");
                return;
            }
            middle = Some(i as u8 + b'A');
        }
    }

    let mut front = String::new();
    for (i, f) in freq.iter().enumerate() {
        for _ in 0..f / 2 {
            front.push((i as u8 + b'A') as char);
        }
    }

    let back: String = front.chars().rev().collect();
    let mut output = String::new();
    write!(output, "{}", front).unwrap();

    if middle.is_some() {
        write!(output, "{}", middle.unwrap() as char).unwrap();
    }

    write!(output, "{}", back).unwrap();
    println!("{}", output);
}
