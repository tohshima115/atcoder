use proconio::input;

fn main() {
    input! {
        mut r: i64,
        mut c: i64,
    }
    r -= 8;
    c -= 8;
    let mut ans = false;
    let max = r.abs().max(c.abs());
    if max % 2 == 0 {
        ans = true;
    }
    println!("{}", if ans {"white"} else {"black"});
}
