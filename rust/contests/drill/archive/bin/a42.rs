use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans: String = s.chars().map(
        |x| if x.is_ascii_uppercase() {
            x.to_ascii_lowercase()
        } else {
        x.to_ascii_uppercase()
        }
    ).collect();
    println!("{}", ans);
}
