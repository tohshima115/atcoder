use proconio::input;

fn main() {
    input! {
        s: String,
        a: usize,
        b: usize
    }
    let mut chars: Vec<char> = s.chars().collect();
    let sa = chars[a -1];
    let sb = chars[b -1];
    chars[a - 1] = sb;
    chars[b - 1] = sa;
    let ans: String = chars.iter().collect();
    println!("{}", ans);
}
