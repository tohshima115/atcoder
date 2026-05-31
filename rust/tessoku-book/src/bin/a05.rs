use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    }
    let mut ans: i64 = 0;
    for red in 0..n{
        if k - red >= 2 {
            if k - red <= n {
                ans += k - red -1;
            }elif k - red >= 2 * n{
                ans += 2 * n - (k - red) + 1
            }
        }
    }
    println!("{}", ans);
}
