use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mid: f64 = if n % 2 == 0 {
        (a[n / 2] as f64 + a[n / 2 + 1] as f64) / 2.0;
    }else{
        a[(n+1)/2] as f64;
    };
    println!("{}", mid);
}
