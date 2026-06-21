// C-12: 区間加算の最大値（いもす法：差分 + 累積和）
// 認識トリガー: 「区間に一律加算が大量」+「最後にまとめて結果」→ いもす法
// diff[l]+=1, diff[r+1]-=1。最後に累積和を取りながら max。diff は n+2 確保

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ops: [(usize, usize); m],
    }
    let _ = (n, m, ops);
    // TODO
}
