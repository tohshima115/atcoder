use proconio::input;

fn main() {
    input! {
        n: usize,
        takahashi: [(i64, usize);n],
        q: usize,
        t: [usize;q],
    }
    let ls: Vec<usize> = takahashi.iter().map(|&(_h, l)| l).collect();
    let mut smax = vec![0i64; n];
    smax[n-1] = takahashi[n-1].0;
    for i in (0..n-1).rev() {
        smax[i] = takahashi[i].0.max(smax[i+1]);
    }
    for &ti in &t {
        let i = ls.partition_point(|&l| l <= ti );
        println!("{}", smax[i]);
    }
}
