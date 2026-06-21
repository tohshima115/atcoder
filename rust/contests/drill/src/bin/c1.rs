// C-1: ペアの差を最小に（貪欲：ソートして隣どうし）
// 認識トリガー: 「差・距離の合計を最小化」+「自由に組み合わせ」→ ソート＋隣接ペア
// 合計は最大 1e14 → i64

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let _ = (n, a);
    // TODO: ソートして step_by(2) で a[i+1]-a[i] を足す
}
