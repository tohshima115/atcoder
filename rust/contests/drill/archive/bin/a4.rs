use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans: i64 = 0;
    for c in s.chars(){
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' 
            => ans += 1, 
            _=>{} 
        }
    }
    println!("{}", ans);
}
