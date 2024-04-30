
fn main() {
    let n = 1000;
    let r5: f64 = 2.23606798;
    let sr5: f64 = 1.0 - r5;
    let br5: f64 = 1.0 + r5;
    let fib = (br5.powi(n) - sr5.powi(n)) / (2.0_f64.powi(n) * r5);
    println!("{}", fib);
}

