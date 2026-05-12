use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let ans = if 400 % n == 0 { 400 / n } else {-1};
    println!("{}", ans)
}
