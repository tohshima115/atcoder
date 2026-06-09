// 練習ポイント: O(N^2)全探索の見抜き（差がK以下の最長区間）C頻出
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: usize,
        k:i64,
        a:[i64; n]
    }
    let mut cnt: usize = 0;
    for i in 0..n{
        let mut max = a[i];
        let mut min = a[i];
        for j in i..n{
            max = max.max(a[j]);
            min = min.min(a[j]);
            if max - min <= k{
                cnt = cnt.max(j - i + 1);
            }else {
                break;
            }
        }
    }
    println!("{}", cnt);
    // TODO: ここに解答を書く
}
