use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }
    let mut teeth = vec![true; n+1];
    for i in 0..q{
        teeth[t[i]] ^= true;
    }
    let mut ans = 0;
    for i in 1..=n{
        if teeth[i]{
            ans += 1;
        }
    }
    println!("{}", ans);
}
