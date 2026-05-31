use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = false;
    let num:i64 = &s.to_num
    if num >= 1 && num <= 349 && num != 316 {
        ans = true
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
