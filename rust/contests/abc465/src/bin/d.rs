use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [(i64, i64, i64);t]
    }
    for &(mut x,mut y, k) in &case{
        let mut cnt: i64 = 0;
        while x != y {
            if x > y {
                x = x / k;
            }else {
                y = y / k;
            }
            cnt += 1;
        }
        println!("{}", cnt)
    }
}
