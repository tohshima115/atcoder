#![allow(unused)]
// 2Dグリッド克服ドリル S1: 矩形和クエリ
// 入力: H W / H行×W列の整数 / Q / 各クエリ "r1 c1 r2 c2"(1始まり)
// 出力: 各クエリの矩形和
// 機構: notes/topics/2d-grid-and-prefix.md の ④（2次元累積和）
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[i64;w];h],
        q: usize,
        q_set: [(usize,usize,usize,usize);q]
    }
    let mut g_sum: Vec<Vec<i64>> = vec![vec![0i64;w+1];h+1];
    for i in 1..=h {
        for j in 1..=w {
            g_sum[i][j] = g_sum[i][j-1] + g[i-1][j-1];
        }
    }
    for j in 1..=w{
        for i in 1..=h{
            g_sum[i][j] += g_sum[i-1][j]; 
        }
    }
    for &(r1, c1, r2, c2) in &q_set{
        let ans = g_sum[r2][c2] - g_sum[r1-1][c2] - g_sum[r2][c1-1] + g_sum[r1-1][c1-1];
        println!("{}", ans);
    }
    // TODO: 自分で実装する
}
