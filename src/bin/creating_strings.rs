fn main() {
    let mut tokens = Scanner::default();
    let s: String = tokens.next();

    let mut v = s.chars().collect::<Vec<char>>();
    v.sort();

    let mut r: Vec<String> = Vec::new();
    r.push(v.iter().collect());
    while next_permutation(&mut v) {
        r.push(v.iter().collect());
    }

    println!("{}", r.len());
    r.into_iter().for_each(|x| {
        println!("{}", x);
    });
}

fn next_permutation(v: &mut [char]) -> bool {
    let n = v.len();
    for i in (0..n - 1).rev() {
        if v[i] < v[i + 1] {
            for j in (i + 1..n).rev() {
                if v[i] < v[j] {
                    v.swap(i, j);
                    break;
                }
            }
            v[i + 1..n].reverse();
            return true;
        }
    }
    false
}
 
#[derive(Default)]
struct Scanner {
    buffer: std::collections::VecDeque<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                return token.parse().ok().expect("PARSE ERROR");
            }
            let mut input = String::new();
            let mut reader = std::io::BufReader::new(std::io::stdin());
            std::io::Read::read_to_string(&mut reader, &mut input).expect("READ ERROR");
            self.buffer = input.split_ascii_whitespace().map(String::from).collect();
        }
    }
}

