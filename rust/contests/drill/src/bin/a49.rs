use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let sum = a.iter().sum::<i64>();
    let max = *a.iter().max().unwrap();
    println!("{}", sum);
    println!("{}", max);
}
