use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let ans: i64 = (n + k - 1) / k;
    println!("{}", ans);
}
