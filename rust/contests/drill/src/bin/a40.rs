use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }
    a.rotate_left(k);
    let ans: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
