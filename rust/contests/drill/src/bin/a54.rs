use proconio::input;

fn main() {
    input! {
        s: String,
        c: char,
    }
    let count= s.chars().filter(|&x| x==c).count();
    println!("{}", count);
}
