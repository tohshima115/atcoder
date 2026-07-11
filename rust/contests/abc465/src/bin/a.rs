use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    println!("{}", if 3 * a > 2 * b {"Yes"} else {"No"});
}
