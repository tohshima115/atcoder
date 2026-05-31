use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64, n]
    }
    let ans = if n == 1 {
        0
    }else{
        let mut max:i64 = 0;
        for i in 0..(a.len()-1){
            if (a[i] - a[i+1]) * (a[i] - a[i+1]) > max{
                max = (a[i] - a[i+1]) * (a[i] - a[i+1])
            };
        };
        sqer(max)
    };
    println!("{}", ans);
}
