use proconio::input;

fn main() {
    input! {
        s: String,
        l: usize,
        r: usize,
    }
    let c: Vec<char> = s.chars().collect();
    let ans: String = c[l-1..r].iter().collect();
    println!("{}", ans);
}
