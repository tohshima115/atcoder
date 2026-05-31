use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i32,
    }
    let mut sigma = 0;
    for i in 0..=m {
        sigma += n.pow( i as u32);
    }
    let ans = if sigma > 10 ** 9 {"inf"} else {sigma};
    println!("{}", ans)
}
