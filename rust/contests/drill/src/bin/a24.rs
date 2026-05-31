use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let ans: i64 = n.to_string().chars().map(|c: char| c.to_digit(10).unwrap() as i64).max().unwrap();
    println!("{}", ans);
}
