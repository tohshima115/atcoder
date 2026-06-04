// 練習ポイント: 参照/deref 総合（Vec<(String,i64)> 最高得点者）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。
use proconio::input;

fn main() {
    input! {
        n: usize,
        student: [(String, i64); n]
    }
    let best = student.iter().map(|(_, s)| *s).max().unwrap();
    let ans = student.iter()
    .filter(|(_, s)| *s == best)
    .map(|(name,_)| name)
    .min()
    .unwrap();
println!("{}", ans);
}
