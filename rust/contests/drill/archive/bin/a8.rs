use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mid: String = s[1..4].parse().unwrap();
    println!("{}", mid)
}

// テストして気がついた。というか、もともと怪しいけどテストで見ればいいやと思った。けど、s[2..4]って最初書いてた。
// これ慣れないなあ