use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let vec :Vec<usize> = (1..=n).collect();
    let s: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" "));
}
