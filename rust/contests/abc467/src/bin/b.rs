use proconio::input;

fn main() {
    input! {
        n: usize,
        pay: [(i64,i64,String);n],
    }
    let mut son: i64 = 0;
    for (a, b, s) in &pay{
        if s == "keep" {
            son += b - a; 
        }
    }
    println!("{}", son);
}
