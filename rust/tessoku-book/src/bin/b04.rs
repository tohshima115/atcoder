use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let mut ans: i64 = 0;
    for (i, c) in n.chars().enumerate(){
        if c == '1' {
            ans += 1 << (n.len() -1 - i)
        }
    }
    println!("{}", ans);
}
