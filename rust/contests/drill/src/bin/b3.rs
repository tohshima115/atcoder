use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [i64; n],
    }
    p.sort();
    let ans = p[p.len() - k];
    println!("{}", ans);
}
