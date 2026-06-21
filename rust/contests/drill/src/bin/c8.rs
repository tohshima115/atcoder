// C-8: 和が K の倍数になる区間の数（累積和 + 余り + カウント）
// 認識トリガー: 「和が K の倍数の区間の数」→ 累積和を取り「同じ余りの組」を数える
// cnt に余りの出現回数。S_0=0 を最初に登録。答えは 1e10 級 → i64

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let _ = (n, k, a);
    let _cnt: HashMap<i64, i64> = HashMap::new();
    // TODO
}
