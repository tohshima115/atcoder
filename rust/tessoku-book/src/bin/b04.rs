use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let bit[i64] = &n.iter.
    let mut ans = i64::new;
    for i in 0..bit.length{
        if bit[i] == 1 {
            ans += 1 << i ;
        }
    }
    println!("{}", ans);
}
