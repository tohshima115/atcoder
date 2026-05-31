use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    }
    let mut ans:i64 = 0;
    let mut isInf: bool = false;
    for i in 0..=m{
        ans += n.pow(i as u32);
        if ans > 10_i64.pow(9 as u32){
            isInf = true;
            break;
        };
    };
    println!("{}", if isInf {"inf"} else {ans});
}
