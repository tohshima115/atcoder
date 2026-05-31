use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans:String = s[..1].parse().unwrap();
    println!("{}", ans);
}

// 試行錯誤
// .parseを最初.charsにしてミスってた
// a2.rsを確認して解いた
// 何も見ずには解けてない感じですね