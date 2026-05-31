use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans: String = String::new();
    for i in (0..10).rev(){
        let bit = (n / (1 << i)) % 2;
        ans.push_str(&bit.to_string());
    }
    println!("{}", ans);
}
