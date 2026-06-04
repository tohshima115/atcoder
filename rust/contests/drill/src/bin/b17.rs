// 練習ポイント: BTreeMap で昇順集計（entry + 自動ソート反復）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut list: BTreeMap<i64, i64> = BTreeMap::new();
    for &key in &a {
        *list.entry(key).or_default() += 1;
    }
    for (&key, &value) in &list{
        println!("{} {}", key, value);
    }
    // TODO: ここに解答を書く
}
