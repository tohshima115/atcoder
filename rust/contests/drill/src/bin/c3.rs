// C-3: x 以上の最小の要素（配列への二分探索 / abc463-C の型）
// 認識トリガー: 「ソートできる」+「同じ配列に何度も問い合わせ」→ ソート＋二分探索
// partition_point(|&v| v < x) で位置。i == n なら -1

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        x: [i64; q],
    }
    let _ = (n, a, q, x);
    // TODO
}
