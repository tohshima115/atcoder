use proconio::input;

fn main() {
    let mut ans: Vec<i64> = vec![];
    for _i in 0..100{
        input! {
            a: i64,
        }
        ans.push(a);
        if a == 0 {
            break;
        }
    }
    ans.reverse();
    for &a in & ans{
        println!("{}", a);
    }
}
