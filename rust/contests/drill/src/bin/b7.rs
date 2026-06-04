// 練習ポイント: 文字列走査・カウント（隣り合う同じ文字）
// 入力の読み方は b1〜b5 と同様に proconio の input! を使う。

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let vec: Vec<char> = s.chars().collect(); 
    let ans: usize = (0..vec.len()-1).filter(|&x| vec[x] == vec[x+1]).count();
    println!("{}", ans);
    // TODO: ここに解答を書く
}
