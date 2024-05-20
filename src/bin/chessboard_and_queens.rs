const BOARD_SIZE: usize = 8;
 
fn main() {
    let board: Vec<Vec<bool>> = io::stdin()
        .lock()
        .lines()
        .take(BOARD_SIZE)
        .map(|line| line.unwrap().trim().chars().map(|c| c == '.').collect())
        .collect();
    println!("{}", solve(0, &mut [0usize; 8], &board));
}
 
fn solve(c: usize, p: &mut [usize; 8], b: &Vec<Vec<bool>>) -> u64 {
    if c == 8 {
        return 1_u64;
    }
    let mut t = 0;
    'outer: for r in 0..BOARD_SIZE {
        if !b[r][c] {
            continue 'outer;
        }
        for (i, x) in p.iter().enumerate().take(c) {
            if *x == r {
                continue 'outer;
            }
            let (dx, dy) = (c as i8 - i as i8, r as i8 - *x as i8);
            if dx.abs() == dy.abs() {
                continue 'outer;
            }
        }
        p[c] = r;
        t += solve(c + 1, p, b);
    }
    t
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
