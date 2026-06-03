use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans: i64 = 1;
    for i in (0..18).rev(){
        if n / 10_i64.pow(i as u32) >= 1{
            ans += i;
            break;
        }
    };
    println!("{}", ans);
}
