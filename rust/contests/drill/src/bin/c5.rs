use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut cnt: HashMap<i64, i64> = HashMap::new();
    for &x in &a {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let ans: i64 = cnt.values().map(|&c| c * (c-1) / 2).sum();
    println!("{}", ans);
}
