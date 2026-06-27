use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let east = s.chars().filter(|&c| c == 'E').count();
    let west = s.chars().filter(|&c| c == 'W').count();
    println!("{}", if east > west {"East"} else {"West"});
}
