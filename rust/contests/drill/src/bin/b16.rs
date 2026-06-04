// 練習ポイント: BTreeMap の範囲クエリ range(x..).next()
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        event: [(i64, i64); n],
        x: [i64; q]
    }
    let event_sort: BTreeMap<i64, i64> = event.iter().copied().collect();
    for i in 0..q {
        let ans = match event_sort.range(x[i]..).next() {
            Some((_k, &v)) => v,
            None => -1,
        };
        println!("{}", ans);
    }
    // TODO: ここに解答を書く
}
