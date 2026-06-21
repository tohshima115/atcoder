// C-9: 和が K 以下の最長区間（尺取り法）
// 認識トリガー: 「連続部分列」+「条件以下の最長」+「全部正で単調」→ 尺取り
// r を進めて sum>k なら l を進める。l,r は各 N 回まで → O(N)。K は 1e18 → i64

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let _ = (n, k, a);
    // TODO
}
