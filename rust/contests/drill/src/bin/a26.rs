use num_traits::float;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[i64; n],
    }
    let sum = a.iter().sum::<i64>();
    let ave:float:: = sum / (n as i64);
    println!("{}", ave)
}
