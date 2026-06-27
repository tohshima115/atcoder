use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut ans = false;
    let mut seen: HashSet<i64> = HashSet::new();
    for &x in &a {
        if seen.contains(&x) {
            ans = true;
        }else {
            seen.insert(k-x);
        }
    }
    println!("{}", if ans {"Yes"} else {
        "No"
    });
}
