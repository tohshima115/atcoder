use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let vec: Vec<i64> = (1..=n).collect();
    let count = vec.iter().filter(|&&x| x % 3 != 0 && x % 5 != 0).count();
    println!("{}", count);
}
