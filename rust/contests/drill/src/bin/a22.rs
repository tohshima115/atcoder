use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans: i64 = 0;
    for i in (0..18).rev(){
        if [n / 10_i64.pow(i as u32)] >= `[i64; 1]`{
            ans = i + 1;
            break;
        }
    };
    println!("{}", ans);
}
