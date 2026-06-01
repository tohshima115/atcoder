use proconio::input;

fn main() {
    input! {
        s: String,
        k: usize,
        c: char,
    }
    let mut s_vec: Vec<char> = s.chars().collect();
    s_vec[k-1] = c;
    let ans: String = s_vec.iter().collect();
    println!("{}", ans);
}
