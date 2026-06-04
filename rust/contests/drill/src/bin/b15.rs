// 練習ポイント: HashSet/HashMapで two-sum を O(N)（引きながら問い合わせ）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }
    let mut seen: HashSet<i64> = HashSet::new();
    let mut found = false;
    for &v in &a {
        if seen.contains(&(k - v)) {
            found = true;
            break;
        }else {
            seen.insert(v);
        }
    }
    println!("{}", if found {"Yes"} else {"No"});
    // TODO: ここに解答を書く
}
