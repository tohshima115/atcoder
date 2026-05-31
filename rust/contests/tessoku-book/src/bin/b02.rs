use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let mut ans = false;
    for v in a..=b{
        if 100 % v == 0{
            ans = true;
            break;
        }
    }
    println!("{}", if ans == true {"Yes"} else {"No"});
}
