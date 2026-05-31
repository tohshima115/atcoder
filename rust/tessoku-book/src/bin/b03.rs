use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = false;
    let a_set: HashSet<i64> = a.iter().cloned().collect();
    'outer: for &v1 in &a{
        for &v2 in &a{
            if a_set.contains(&(& 1000 - (v1 + v2))){
                ans = true;
                break `outer;
            }
        } 
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
