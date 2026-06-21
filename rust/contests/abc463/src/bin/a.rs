use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }
    let mut ans = false;
    if x * 9 == y * 16 {
        ans = true;
    }
    println!("{}", if ans {"Yes"} else {
        "No"
    });
}
