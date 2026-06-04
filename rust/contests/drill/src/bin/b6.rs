// 練習ポイント: 二重ループ全探索（和が K になるペア）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }
    let mut ans: usize = 0;
    for i in 0..n-1{
        let cnt: usize = (i+1..n).filter(|&x| a[x] + a[i] == k).count();
        ans += cnt;
    }
    println!("{}", ans);
}
