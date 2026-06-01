use proconio::input;

fn main() {
    input! {
        n:usize,
        s: [String; n],
    }
    let mut ans: i64 = 0;
    let mut list: Vec<String> = s.clone();
    list.sort();
    list.dedup();
    for v in &list{
        let cnt: usize = s.iter().filter(|&x| x == v).count();
        if cnt as i64 > ans {
            ans = cnt as i64;
        }
    }
    println!("{}", ans);
}
