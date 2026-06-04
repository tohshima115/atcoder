use std::collections::HashMap;

// 練習ポイント: 累積和＋seen-so-far（和がKの区間数）C頻出
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }
    let mut ans:i64 = 0;
    let mut a_sum: Vec<i64> = vec![0i64; n+1];
    for i in 0..n {
        a_sum[i+1] = a_sum[i] + a[i]
    }
    let mut count: HashMap<i64,i64> = HashMap::new();
    for j in 0..=n {
        let s = a_sum[j];
        ans += *count.get(&(s - k)).unwrap_or(&0);
        *count.entry(s).or_default() += 1;
    }
    println!("{}", ans);
    // TODO: ここに解答を書く
}
