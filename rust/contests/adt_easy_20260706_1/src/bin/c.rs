use proconio::input;

fn main() {
    input! {
        x: String,
    }
    let ans = x.trim_end_matches('0').trim_end_matches('.');
    println!("{}", ans);
}
