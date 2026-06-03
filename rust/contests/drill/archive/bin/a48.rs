use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    a.insert(0, -1);
    a.push(1001);
    let mut ans: i64 = 0;
    for i in 1..=n{
        if a[i-1] != a[i] && a[i] != a[i+1] {
            ans += 1;
        }
    }
    // let debug:Vec<String> = a.iter().map(|x| x.to_string()).collect();
    // println!("{}", debug.join(" "));
    println!("{}", ans);
}
