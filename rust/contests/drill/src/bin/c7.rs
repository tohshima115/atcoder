use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut s = vec![0i64; n + 1];
    for i in 0..n { 
        s[i + 1] = s[i] + a[i]; 
    }
    for &(l,r) in &lr {
        println!("{}", s[r] - s[l] );
    }
}
