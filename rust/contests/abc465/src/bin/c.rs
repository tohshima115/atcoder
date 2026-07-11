use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }
    let str: Vec<char> = s.chars().collect();
    let mut a: Vec<i64> = vec![0i64;n];
    for j in 0..n{
        a[j] = (j + 1) as i64;
    }
    for i in 1..n{
        if str[i] == 'x' {

        }else {
            a[0..=i].reverse();
        }
    }
    let ans: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
