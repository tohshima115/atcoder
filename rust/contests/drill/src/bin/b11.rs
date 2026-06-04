// 練習ポイント: ソート＋貪欲（詰め込める個数の最大）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        mut weight: [i64; n]
    }
    weight.sort();
    let mut w_sum: i64 = 0;
    let mut cnt: i64 = 0;
    for &w_i in &weight{
        w_sum += w_i;
        if w_sum <= w{
            cnt += 1;
        }else {
            break;
        }
    }
    println!("{}", cnt);
    // TODO: ここに解答を書く
}
