use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64; n],
    }
    let ans = a.iter().sum::<&i64>() - a.iter().max();
    println!("{}", ans);
}
