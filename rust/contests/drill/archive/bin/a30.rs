use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    // let mut count:i64 = 0;
    // for &v in &a{
    //     if v >= k {
    //         count += 1;
    //     };
    // };
    let count: usize = a.iter().filter(|&&x| x >= k).count();
    println!("{}", count);
}
