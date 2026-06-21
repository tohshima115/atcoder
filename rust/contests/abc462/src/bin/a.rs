use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans: String = s.chars().filter(|x| x.is_ascii_digit()).collect();
    println!("{}", ans);
}
