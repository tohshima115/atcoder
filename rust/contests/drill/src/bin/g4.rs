#![allow(unused)]
use itertools::Itertools;
// 2Dグリッド克服ドリル G4: 上下左右4近傍の和（境界処理）
// 入力: H W / H行×W列の整数
// 出力: 各セルの上下左右(存在するものだけ)の和を、同じ形(H行×W列)で
// 機構: ① / 境界チェック（i>0, i+1<h, ...）
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[i64; w]; h]
    }
    let mut frame = vec![vec![0i64; w+2]; h+2];
    for i in 0..h{
        for j in 0..w{
            frame[i+1][j+1] = g[i][j]
        }
    }
    for i in 0..h{
        let mut ans = vec![0i64;w];
        for j in 0..w{
            ans[j] = frame[i][j+1] + frame[i+1][j] + frame[i+1][j+2] + frame[i+2][j+1];
        }
        println!("{}", ans.iter().join(" "));
    }
    // TODO: 自分で実装する
}
