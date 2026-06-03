use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let count = a.iter().filter(|&&x| x % 2 == 0).count();
    println!("{}", count);
}
