use std::{i64::MAX};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut point: [(i64, i64); n]
    }
    let mut cnt: i64 = 0;
    let mut min_x: i64 = MAX;
    point.sort_by(|(_, a), (_, b)| a.cmp(b));
    for &(x, _y) in &point{
        if x <= min_x {
            cnt += 1;
            min_x = x;
        }
    }
    println!("{}", cnt);
}
