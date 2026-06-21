// C-7: 区間和クエリ（累積和）
// 認識トリガー: 「区間の和を何度も聞かれる」→ 累積和を前計算
// pre[0]=0, pre[i+1]=pre[i]+a[i]。区間[l,r](1-indexed)= pre[r]-pre[l-1]。i64

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let _ = (n, a, q, lr);
    // TODO
}
