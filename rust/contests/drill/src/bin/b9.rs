// 練習ポイント: 累積和（区間の合計）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        q_vec: [(usize, usize); q]
    }
    let mut a_sum = vec![0i64; n+1];
    for i in 0..n{
        a_sum[i+1] = a[0..i+1].iter().sum::<i64>();
    }
    for &(l, r) in &q_vec{
        let ans = a_sum[r] - a_sum[l-1];
        println!("{}", ans); 
    }
    // TODO: ここに解答を書く
}
