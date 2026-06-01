use proconio::input;

fn main() {
    input! {
        s: String,
    }
    // let mut c: Vec<char> = s.chars().collect();
    // let c_rev: Vec<char> = c.reverse();
    // let ans: bool = c == c_rev;
    // let c: Vec<char> = s.chars().collect();
    // let c_rev: Vec<char> = c.iter().rev().copied().collect();
    // let ans: bool = c == c_rev;
    let rev: String = s.chars().rev().collect();
    let ans: bool = s == rev;
    println!("{}", if ans {"Yes"} else {"No"});
}
