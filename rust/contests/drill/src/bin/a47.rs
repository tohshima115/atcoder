use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    a.dedup();
    println!("{}", a.len());
}
