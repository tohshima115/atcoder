// 練習ポイント: HashMap集計（最頻値を全部・昇順）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut count: BTreeMap<i64, i64> = BTreeMap::new();
    for &k in &a {
        *count.entry(k).or_default() += 1;
    }
    let max = *count.values().max().unwrap();
    let mut list: Vec<i64> = Vec::new();
    for (&k, &v) in &count {
        if v == max {
            list.push(k);
        }
    }
    let ans: Vec<String> = list.iter().map(|&x| x.to_string()).collect();
    println!("{}", ans.join(" ")); 
    // TODO: ここに解答を書く
}
