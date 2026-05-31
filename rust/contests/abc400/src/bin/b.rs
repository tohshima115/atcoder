use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    }
    let mut ans:i64 = 0;
    for i in 0..=m{
        ans += n.pow(i as u32);
        if ans > 10_i64.pow(9 as u32){
            println!("inf");
            break;
        }else if i == m{
            print!("{}", ans);
        };
    };
}
