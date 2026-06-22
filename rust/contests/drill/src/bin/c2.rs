// C-2: 待ち時間の合計を最小に（貪欲：SPT 昇順）
// 認識トリガー: 「完了時刻／待ち時間の合計を最小化」→ 処理時間の昇順
// 累計(acc)と総和(sum)を回す。合計は 1e14 級 → i64

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [i64; n],
    }
    t.sort();
    let mut ans: i64 = 0;
    for i in 0..n {
        ans += t[i] * (n as i64 - i as i64);
    }
    println!("{}", ans);
}
