use proconio::input;

fn main() {
    input! {
        s: String,
        a: usize,
        b: usize
    }
    let mut chars: Vec<char> = s.chars().collect();
    chars.swap(a-1,b-1);
    let ans: String = chars.iter().collect();
    println!("{}", ans);
}
