use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ball: [(i64, i64);n]
    }
    let mut list: Vec<i64> = vec![];
    for i in 1..=m{
        let push: i64 = ball.iter().filter(|&(c,_s)| *c == (i as i64)).map(|&(_c,s)| s).max().unwrap_or(-1);
        list.push(push);
    }
    let ans: Vec<String> = list.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "))
}
