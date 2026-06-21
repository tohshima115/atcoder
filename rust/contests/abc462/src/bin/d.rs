use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        time: [(usize, usize); n]
    }
    let mut timer: Vec<i64> = vec![0i64;n]; 
    for &(s ,t) in &time {
        timer[s] += 1;
        timer[t - d] -= 1;
    }
    for i in 0..n {
        timer[i] += timer[i-1];
    }
    println!("{}", n);
}
