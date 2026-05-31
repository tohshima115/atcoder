use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[i64; n],
    }
    let sum = a.iter().sum::<i64>();
    let ave: f64 = sum as f64 / n as f64;
    println!("{:.2}", ave)
}
