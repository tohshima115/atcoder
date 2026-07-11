use std::io::{stdin, stdout, BufReader, Write};
 
use proconio::{input, source::line::LineSource};
 
fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
 
    input! {
        from &mut source,
        n: usize,
    }

    let mut len: usize = 2;
    let mut ans: usize = 0;
    for i in 1..n {
        let mut response = true;
        while response {
            if len <= n {
                if len == i {
                    len += 1;
                }
                println!("? {} {}", i, len);
                stdout().flush().unwrap();
                input! {
                    from &mut source,
                    y: String,
                }
                if y == "No" {
                    response = false;
                }else {
                    len += 1;
                }
            }else {
                response = false;
            }
        }
        ans += len - i - 1;
    }
    println!("! {}", ans);
}
