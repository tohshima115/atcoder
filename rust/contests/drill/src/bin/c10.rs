// C-10: 迷路の最短手数（BFS / グリッド）
// 認識トリガー: 「グリッド」+「最短手数(重み1)」→ BFS
// dist を -1 初期化、VecDeque。範囲外は一度 i64 にして判定。訪問済みチェック必須

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let _ = (h, w, grid);
    let _q: VecDeque<(usize, usize)> = VecDeque::new();
    // TODO: S と G の座標を探してから BFS
}
