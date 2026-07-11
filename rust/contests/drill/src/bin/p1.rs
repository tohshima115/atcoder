#![allow(unused)]
// 2Dグリッド克服ドリル P1: 2次元いもす法（矩形に区間加算）
// 入力: H W / Q / 各操作 "r1 c1 r2 c2"(1始まり) で矩形領域に +1
// 出力: 全操作後のグリッド（H行×W列、空白区切り）
// 機構: いもす（4隅に+1/-1を置き、最後に2次元累積和 ④）
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[i64; w];h],
        q: usize,
        query: [(usize, usize, usize, usize);q]
    }
    let mut sum_grid: Vec<Vec<i64>> = vec![vec![0i64;w+1];h+1];
    for i in 1..=h{
        for j in 1..=w{
            sum_grid[j][i] = sum_grid[j-1][i] + sum_grid[j][i-1] - sum_grid[j-1][i-1] + g[j-1][i-1];
        }
    }
    // TODO: 自分で実装する
}
