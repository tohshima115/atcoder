use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32;n]
    }
    let mut ans = false;
    for i in 0..n{
        if x[i] >= (0 as i32) {
            ans = true;
            break;
        }
    }
    println!("{}", if ans {"No"} else {"Yes"});
}
