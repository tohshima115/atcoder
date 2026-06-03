use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans: u32 = 0;
    for x in s.chars(){
        let n: u32 = x.to_digit(10).unwrap();
        ans += n;
    }
    println!("{}", ans);
}

// ヒント見ながら解けたけどやなあ。
// これは結構自信ないのと、割と覚える方法が長くて何回かやらないと覚えられなさそう。
// でも結構出そう。
// なんでこんな簡単そうな操作が大変なんだ。。。