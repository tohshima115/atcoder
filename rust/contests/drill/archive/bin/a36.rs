use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }
    let mut vec: Vec<usize> = (1..=n).collect();
    vec[l-1..r].reverse();
    let s: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" "));
}
