use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut vec: Vec<char> = s.chars().collect();
    vec.reverse();
    let ans: String = vec.iter().collect();
    println!("{}", ans);
}
