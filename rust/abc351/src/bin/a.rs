use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n]
    }
    let mut ans: i64 = -1;
    for i in 2..=n {
        if h[1] > h[i]
        ans = i;
        break;
    }
    println!("{}", ans);
}
