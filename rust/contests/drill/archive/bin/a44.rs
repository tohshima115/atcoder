use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let ans = if a > b {
        "Greater"
    }else if a < b {
        "Less"
    }else if a == b {
        "Equal"
    }else{
        unreachable!()
    };
    println!("{}", ans);
}
