use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    println!("{} {}", s, t);
}

// ヒント見た。let ans = format!("{} {}", s, t);が一番良さそう
// これはまあ一回覚えたら忘れなさそうだな。
// "{} {}"で２つ入れる方法は覚えておきたい。結構使いそう