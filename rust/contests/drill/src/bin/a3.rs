use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [i64; n]
    }
    let min = t.iter().min().unwrap();
    println!("{}", min)
}
