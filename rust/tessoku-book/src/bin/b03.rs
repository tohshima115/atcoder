use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = false;
    'outer: for i in 0..n-2{
        for j in i..n-1{
            for k in j..n{
                if a[i] + a[j] + a[k] == 1000{
                    ans = true;
                    break 'outer;
                }
            }
        }
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
