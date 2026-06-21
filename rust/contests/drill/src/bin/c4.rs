// C-4: 荷物の分割で最大負荷を最小に（答えで二分探索）
// 認識トリガー: 「最大を最小化」+「ある値で可能か?の判定が簡単」→ 答えで二分探索
// 判定: 容量 x で左から貪欲に詰めて使うトラック数 <= K か
// lo = max(w), hi = sum(w) から while lo < hi

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        w: [i64; n],
    }
    let _ = (n, k, w);
    // TODO
}
