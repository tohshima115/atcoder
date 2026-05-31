use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let mut ans = i64::new;
    for (i, c) in n.chars().enumerate(){
        if c == '1' {
            ans += 1 << (n.length - i)
        }
    }
    println!("{}", ans);
}
