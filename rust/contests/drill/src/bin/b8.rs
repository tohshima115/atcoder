// 練習ポイント: シミュレーション（階段の上り下り・クランプ）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: i32,
        m: usize,
        step: [(char, i32); m]
    }
    let mut ans:i32 = 0;
    for (op, x) in &step{
        if *op == 'U' {
            if ans + x < n{
                ans += x;
            }else {
                ans = n;
            }
        }else {
            if ans - x > 0{
                ans -= x;
            }else {
                ans = 0;
            }
        }
    }
    println!("{}", ans);
    // TODO: ここに解答を書く
}