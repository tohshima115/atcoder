use proconio::input;

fn main() {
    input! {
        a: usize,
        d: usize,
    }
    println!("{}", if d >=a {"Yes"} else {"No"});
}
