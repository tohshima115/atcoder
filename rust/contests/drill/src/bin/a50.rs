use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut diff: Vec<i64> = vec![0i64; n-1];
    for i in 0..n-1{
        diff[i] += a[i+1] - a[i];
    }
    let max: i64 = *diff.iter().max().unwrap();
    println!("{}", max);
}
