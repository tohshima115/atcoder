// 練習ポイント: bit 全探索（部分集合の和）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n]
    }
    let mut ans = false;
    for mask in 0u64..(1 << n){
        let mut sum = 0i64;
        for i in 0..n{
            if (mask >> i) & 1 == 1 {
                sum += a[i];
            }
        }
        if sum == k {
            ans = true;
            break;
        }
    }
    println!("{}", if ans {"Yes"} else {
        "No"
    })
    // TODO: ここに解答を書く
}
