use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = false;
    for &v1 in &a{
        for &v2 in &a{
            if v1 > v2 {
                for &v3 in &a{
                    if v2 > v3 && v1 + v2 + v3 == 1000{
                        ans = true;
                        break;
                    }
                }
            }

        }
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
