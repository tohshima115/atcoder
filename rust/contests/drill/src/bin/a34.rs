use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans: i64 = 1;
    for i in 1..=n{
        ans = ans * i;
    };
    println!("{}", ans);
}
