use proconio::input;

fn main() {
    input! {
        s: u32,
    }
    println!("{}", if (200..=299).contains(&s) { "Success" } else { "Failure" });
}
