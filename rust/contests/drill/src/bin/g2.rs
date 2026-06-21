use itertools::Itertools;
// 2Dグリッド克服ドリル G2: 各行の和・各列の和
// 入力: H W / H行×W列の整数
// 出力: 各行の和をH行 → 続けて各列の和を空白区切り1行
// 機構: notes/topics/2d-grid-and-prefix.md の ①（二重ループの向き）
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [[i64; w]; h]
    }
    for i in 0..h{
        println!("{}", g[i].iter().sum::<i64>())
    }
    let mut col_sum: Vec<i64> = vec![0i64;w];
    for i in 0..w{
        for j in 0..h{
            col_sum[i] += g[j][i];
        }
    }
    println!("{}", col_sum.iter().join(" "))
    // TODO: 自分で実装する
}
