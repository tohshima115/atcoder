use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [[char;w];h]
    }
    let mut top = false;
    let mut bottom = false;
    let mut left = false;
    let mut right = false;
    while top {
        for i in 0..w{
            if c[0][i] == '#' {
                top = true;
            }

        }
    }
    println!("{}", n);
}
