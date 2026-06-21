// C-5: 同じ値のペアの数（HashMap カウント）
// 認識トリガー: 「出現回数を数える」「同じ値の組」→ HashMap
// 値ごとに c*(c-1)/2 を足す。答えは 2e10 級 → i64

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let _ = (n, a);
    let _m: HashMap<i64, i64> = HashMap::new();
    // TODO
}
