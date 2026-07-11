use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        l: i64,
        r: i64,
        a: i64,
        b: i64
    }
    let mut now: i64 = a;
    let mut fee: i64 = 0;
    while now < b {
        if l <= now && now < r {
            fee += x;
        }else {
            fee += y;
        }
        now += 1;
    } 
    println!("{}", fee);
}
