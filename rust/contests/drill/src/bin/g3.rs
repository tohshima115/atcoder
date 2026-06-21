#![allow(unused)]
use std::default;

// 2Dグリッド克服ドリル G3: 時計回りに90度回転
// 入力: H W / H行の文字列（長さW）
// 出力: 時計回り90°回転後のグリッド（W行×H列）
// 機構: ① / 新旧の添字対応 new[i][j] = old[H-1-j][i]
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [String;h]
    }
    let grid: Vec<Vec<char>> = g.iter().map(|row| row.chars().collect()).collect();
    let mut ans = vec![vec![' '; h]; w];
    for i in 0..w{
        for j in 0..h{
            ans[i][j] = grid[h-1-j][i];
        }
    }
    for row in &ans {
        let s:String = row.iter().collect();
        println!("{}", s);
    }
    // TODO: 自分で実装する
}
