use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let ans = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).max();
    println!("{}", ans);
}
