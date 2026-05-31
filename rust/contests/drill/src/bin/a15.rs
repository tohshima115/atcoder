use proconio::input;

fn main() {
    input! {
        m: usize,
    }
    let mut season = String::new();
    if 3 <= m && m <= 5 {
        season = "spring".to_string();
    }else if 6 <= m && m <= 8 {
        season = "summer".to_string();
    }else if 9 <= m && m <= 11 {
        season = "autumn".to_string();
    }else{
        season = "winter".to_string();
    }
    println!("{}", season);
}
