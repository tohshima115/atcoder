use proconio::input;

fn main() {
    input! {
        y: i64,
    }
    let ans = (y - 1) / 100;
    println!("{}", ans);
}
