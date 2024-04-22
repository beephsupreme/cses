use std::fmt::Write;
 
fn main() {
    let mut buffer: String = String::new();
    let mut reader = std::io::BufReader::new(std::io::stdin());
    std::io::BufRead::read_line(&mut reader, &mut buffer).unwrap();
    let n: u64 = input.trim().parse().unwrap();
    let mut sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        println!("NO");
        return
    } else {
        sum /= 2;
    }
    for i in (1..=n).rev() {
        if i <= sum {
            sum -= i;
            print!("{i }");
        } else {
            print!(i)
        }
    }
}






    match two_sets(n) {
        Some((a, b)) => {
            writeln!(output, "YES").unwrap();
            writeln!(output, "{}", a.len()).unwrap();
            a.iter().for_each(|e| write!(output, "{} ", e).unwrap());
            writeln!(output, "\n{}", b.len()).unwrap();
            b.iter().for_each(|e| write!(output, "{} ", e).unwrap());
            writeln!(output).unwrap();
        }
        None => writeln!(output, "NO").unwrap(),
    }
    println!("{}", output);
}
 
fn two_sets(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let (mut a, mut b) = (Vec::new(), Vec::new());
    Some((a, b))
}
