// 練習ポイント: 数学・約数列挙（√N ループ）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let ans = (1..=n).filter(|x | n % x == 0).count();
    println!("{}", ans);
    // TODO: ここに解答を書く
}
