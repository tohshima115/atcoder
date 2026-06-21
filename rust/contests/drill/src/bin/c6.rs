// C-6: 和が K になる 2 つ（HashSet で相方探し / two-sum）
// 認識トリガー: 「和が K になるペアの存在」→ 相方 K-x を集合で探す
// 注意: 今の要素を insert する「前」に相方を探す。K は 2e9 → i64

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let _ = (n, k, a);
    let _s: HashSet<i64> = HashSet::new();
    // TODO
}
