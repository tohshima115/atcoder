// C-1: ペアの差を最小に（貪欲：ソートして隣どうし）
// 認識トリガー: 「差・距離の合計を最小化」+「自由に組み合わせ」→ ソート＋隣接ペア
// 合計は最大 1e14 → i64

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    let ans: i64 = a.chunks(2).map(|p| p[1] - p[0]).sum();
    println!("{}", ans);
}
