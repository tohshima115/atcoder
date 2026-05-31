use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans: i64 = 0;
    for i in (0..18).revers(){
        if [n / 10.dat(i)] >= 1{
            ans = i;
            break;
        }
    };
    println("{}", ans);
}
