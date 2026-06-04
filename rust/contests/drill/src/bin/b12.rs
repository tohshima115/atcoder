// 練習ポイント: HashMap 集計（最頻値）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut list: Vec<i64> = a.clone();
    list.sort();
    list.dedup();
    let mut ans: i64 = 0;
    let mut most: usize = 0;
    for &v in &list{
        let cnt = a.iter().filter(|&&x| x == v).count();
        if cnt > most{
            most = cnt;
            ans = v;
        }
    }
    println!("{}", ans);
    // TODO: ここに解答を書く
}
