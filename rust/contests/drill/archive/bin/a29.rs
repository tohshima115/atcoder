use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }
    a.sort();
    let ans: i64 = a[(a.len() - k)..].iter().sum::<i64>();
    println!("{}", ans);
}
