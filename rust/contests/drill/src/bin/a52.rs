use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }
    let pos: Option<usize> = a.iter().position(|&v| v == x);
    match pos {
        Some(i) => println!("{}", i + 1),
        None => println!("-1"),
    }
}
