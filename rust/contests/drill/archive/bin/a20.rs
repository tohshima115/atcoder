use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut max:i64 = 0;
    for i in 0..(a.len()-1){
        let diff = (a[i] - a[i+1]).abs();
        if diff > max {
            max = diff
        }
    };
    println!("{}", max);
}
