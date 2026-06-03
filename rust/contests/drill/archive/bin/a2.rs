use proconio::input;

fn main() {
    input! {
        s: String
    }
    let num: i64 = s[3..].parse().unwrap();
    let ans = 100 <= num && num <= 500;
    println!("{}", if ans {"OK"} else {"NG"})
}
