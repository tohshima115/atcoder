use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut bird: [(i64,usize,i64);n]
    }
    let mut bird_list: HashMap<i64,i64> = HashMap::new();
    for &(a, _d, _b) in &bird{
        *bird_list.entry(a).or_insert(0) += 1;
    }
    bird.sort_by(|(_, a, _), (_, b,_)| a.cmp(b));
    let mut bird_cnt: Vec<(usize,usize)> = vec![];
    for &(a, d, b) in &bird {
        if *bird_list.get(&a).unwrap() == 1 {
            bird_list.remove(&a);
        }else {
            *bird_list.entry(a).or_insert(0) -= 1;
        }
        *bird_list.entry(b).or_insert(0) += 1;
        bird_cnt.push((bird_list.len(),d));
    }
    let ls: Vec<usize> = bird.iter().map(|&(_a, d, _b)| d).collect();
    for x in 0..m {
        let i = ls.partition_point(|&l| l <= x );
        println!("{}", bird_cnt[i].0);
    }
    println!("{}", n);
}
