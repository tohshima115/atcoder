use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut isdebu = false;
    if w * 10000 >= 25 * h * h {
        isdebu = true;
    }
    println!("{}", if isdebu {"Yes"} else {"No"});
}
