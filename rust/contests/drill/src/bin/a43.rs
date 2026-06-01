use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, i64); q],
    }
    let mut a: Vec<i64> = vec![0i64; n];
    for (x, v) in queries {
        a[x -1] += v;
    }
    let ans: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
