use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let ans = (a - b).abs();
    println!("{}", ans);
}
