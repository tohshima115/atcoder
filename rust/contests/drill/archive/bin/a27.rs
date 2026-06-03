use proconio::input;

fn main() {
    input! {
        b: i64,
        h: i64,
    }
    let size: f64 = b as f64 * h as f64 / 2.0;
    println!("{:.1}", size);
}
