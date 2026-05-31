use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    println!("{}", if s == t {"Yes"} else {"No"})
}

// こんな簡単でいいんか？って思いながらやった
// なんか落とし穴あるんかなと思ったらそのまま行けた