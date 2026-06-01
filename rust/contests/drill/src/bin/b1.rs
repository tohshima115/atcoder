use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }
    let mut light: Vec<bool> = vec![false;n];
    for i in 0..q{
        light[t[i]-1] ^= true;
    }
    let ans = light.iter().filter(|&&x| x).count();
    println!("{}", ans);
}
