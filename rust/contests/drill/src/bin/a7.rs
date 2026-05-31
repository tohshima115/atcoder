use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans: String = s[s.len() - 1 ..].parse().unwrap();
    println!("{}", ans);
}

// 最初こんな感じで１行で書こうとして失敗
// ansを挟めば行けることはわかってたというか、ほぼa6と同じ問題だからできるのは見えてたけど、１行にできないもんかな？
// fn main() {
//     input! {
//         s: String,
//     }
//     println!("{}", s[s.len() - 1 ..].parse().unwrap());
// }
