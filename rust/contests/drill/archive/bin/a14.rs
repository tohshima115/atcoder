use proconio::input;

fn main() {
    input! {
        s: String,
        c: char,
    }
    let mut ans: i64 = 0;
    for x in s.chars(){
        if x == c {ans += 1};
    }
    println!("{}", ans);
}

// なんでだめなのかわかんね