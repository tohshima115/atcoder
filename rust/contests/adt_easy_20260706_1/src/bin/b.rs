use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        k: i64,
        l: [(i64, usize);n]
    }
    let mut sum: i64 = 0;
    for &(p,q) in &l{
        sum += p * (q as i64);
    }
    if sum < s {
        sum += k;
    }
    println!("{}", sum);
}
