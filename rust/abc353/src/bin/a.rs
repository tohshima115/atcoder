use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n]
    }
    let mut ans: i64 = -1;
    for i in 1..n {
        if h[0] < h[i]{
            ans = i as i64 + 1;
            break;
        }
    }
    println!("{}", ans);
}
