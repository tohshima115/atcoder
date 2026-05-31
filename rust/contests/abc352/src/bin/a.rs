use proconio::input;

fn main() {
    input! {
        _n: usize,
        x: i64,
        y: i64,
        z: i64,
    }
    let is_mid:bool = (x < z && z < y) | (y < z && z < x);
    println!("{}", if is_mid {"Yes"} else {"No"});
}
