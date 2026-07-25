use proconio::input;

fn main() {
    input! {
        m: i64,
        d: usize,
        s: String,
    }
    let mut cnt: i64 = m;
    let c: Vec<char> = s.chars().collect();
    for i in 0..m as usize {
        for j in 0..=d {
            if c[0.max(i as i64 - j as i64) as usize] == 'G' || c[(m - 1).min(i as i64 + j as i64) as usize] == 'G'{
                cnt -= 1;
                break;
            }
        }
    }
    println!("{}", cnt);
}
