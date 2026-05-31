use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans: i64 = 0;
    for c in s.chars(){
        match c {
            'a' => ans += 1,
            _=> {}
        };
    };
    println!("{}", ans);
}

// 普通に解けた。
// matchの構文だけちょっと不安だったけど無事問題なく。