use proconio::input;

fn main() {
    input! {
        c: char,
    }
    let ans = match c {
        'a' | 'e' | 'i' | 'o' | 'u' => "vowel",
        'a'..='z' => "consonant",
        '0'..='9' => "digit",
        _=> unreachable!(),
    };
    println!("{}", ans);
}

// ヒント見て解いた。
// match使えるんだってなったな。
// 'a'..='z'とか'0'..='9'とか使えるのなんかすごいけどよしなにやってくれすぎだなって印象。
//         _=> unreachable!(),
// これは書かなあかんのか。それとも最後のdigitを_=> "digit"にしちゃっても競技プログラミング的には大丈夫？