use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans:u32 = s.chars().map(|x: char| x.to_digit(10).unwrap()).sum();
    println!("{}", ans);
}
