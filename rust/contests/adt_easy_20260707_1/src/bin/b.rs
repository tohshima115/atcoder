use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let str: Vec<char> = s.chars().collect();
    let mut ans: i64 = -1;
    for (i, &c) in str.iter().enumerate() {
        if c == 'a' {
            ans = (i + 1) as i64;
        }
    }
    println!("{}", ans);
}
