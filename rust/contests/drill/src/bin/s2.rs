#![allow(unused)]
// 2Dグリッド克服ドリル S2: 固定サイズ k×k の最大和
// 入力: H W k / H行×W列の整数
// 出力: すべての k×k 正方形の和の最大値
// 機構: ④（各窓を累積和で O(1)）
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        g: [[i64; w]; h]
    }
    let mut g_sum: Vec<Vec<i64>> = vec![vec![0i64; w+1]; h+1];
    for i in 1..=h {
        for j in 1..=w {
            g_sum[i][j] = g[i-1][j-1] + g_sum[i][j-1] + g_sum[i-1][j] - g_sum[i-1][j-1];
        }
    }
    let mut ans: i64 = i64::MIN;
    for i in 0..=h-k {
        for j in 0..=w-k{
            let window = g_sum[i+k][j+k] - g_sum[i+k][j] - g_sum[i][j+k] + g_sum[i][j];
            ans = ans.max(window);
        }
    }
    println!("{}", ans);
    // TODO: 自分で実装する
}
