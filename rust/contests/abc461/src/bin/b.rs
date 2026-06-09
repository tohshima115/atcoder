use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut ans = true;
    for i in 0..n {
        if i+1 != b[a[i]-1] {
            ans = false;
            break;
        }
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
