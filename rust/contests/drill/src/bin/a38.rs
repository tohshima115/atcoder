use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }
    // let mut a_bef = a[..k].iter().collect();
    // let a_aft = a[k..].iter().collect();
    // a_bef.push(x);
    // let marge = a_bef + a_aft;
    a.insert(k-1, x);
    let ans: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
