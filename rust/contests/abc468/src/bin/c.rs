use proconio::input;

fn factorial(n: usize) -> i64 {
    let mut result = 1;
    // 1は計算しなくてもいいので2スタート
    for i in 2..=n {
        result *= i as i64
    }
    result
}

fn main() {
    input! {
        n: usize,
        p: [i64;n],
        q: [i64;n],
    }
    let mut p_cnt: i64 = 0;
    let mut q_cnt: i64 = 0;
    for i in 0..n {
        let miman = p[i..].iter().filter(|&x| p[i] > *x).count();
        p_cnt += miman as i64 * factorial(n - i - 1);
    }
    for i in 0..n {
        let miman = q[i..].iter().filter(|&x| q[i] > *x).count();
        q_cnt += miman as i64 * factorial(n - i -1);
    }
    let ans = 0.max(q_cnt - p_cnt - 1);
    println!("{}", ans);
}
