use proconio::input;

fn main() {
    input! {
        n: usize,
        a:[i64; n],
    }
    let ans = if a[0] >= a[n-1] {
        a[0] - a[n-1]
    }else{
        a[n-1] - a[0]
    };
    println!("{}", ans);
}

// これでいいの…？