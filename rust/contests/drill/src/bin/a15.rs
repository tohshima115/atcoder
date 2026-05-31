use proconio::input;

fn main() {
    input! {
        m: usize,
    }
    let season = match m {
        3..=5 => "spring",
        6..=8 => "summer",
        9..=11 => "autumn",
        _=> "winter",
    };
    println!("{}", season);
}
