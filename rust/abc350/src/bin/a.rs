use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let num:i64 = s[3..].parse().unwrap();
    let ans = num >= 1 && num <= 349 && num != 316;
    println!("{}", if ans {"Yes"} else {"No"});
}