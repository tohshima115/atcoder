use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64;n],
    }
    let mut cnt: i64 = 0;
    for i in 0..n-2{
        if a[i] < a[i+1] && a[i+1] > a[i+2] {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
