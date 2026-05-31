use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n]
    }
    let mut ans = false;
    for &v in &a{
        if v == x {ans = true};
        break; 
    };
    println!("{}", if ans == true {"Yes"} else {"No"});
}