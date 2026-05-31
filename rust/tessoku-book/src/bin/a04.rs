use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    for i in 9..0{
        [n / 2 ** i] % 2 = ans[i]
    }
    println!("{}", n);
}
