use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let ans = match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        _=> "five",
    };
    println!("{}", ans);
}

// 瞬殺だったけどこれでいいのか？