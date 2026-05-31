use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans: i64 = 0;
    let mut m: i64 = 0;
    for i in (0..18).rev(){
        let a = (n - m) / 10_i64.pow(i as u32);
        ans += a;
        m += a * 10_i64.pow(i as u32);
    };
    println!("{}", ans);
}
