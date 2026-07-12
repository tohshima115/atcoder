use proconio::input;

fn main() {
    input! {
        s: [i64;8],
    }
    let mut ans = true;
    for i in 1..8 {
        if s[i] < s[i-1] {
            ans = false;
        }
    }
    for i in 0..8 {
        if s[i] % 25 != 0 {
            ans = false;
        }
        if s[i] < 100 || 675 < s[i] {
            ans = false;
        }
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
