use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        v: [[i64; w]; h]
    }
    for i in 0..h{
        let ans = v[i].iter().sum::<i64>();
        println!("{}", ans);
    }
}
