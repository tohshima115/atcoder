use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: i64,
        s: [String; h]
    }
    let mut grid = vec![vec![0i64;w];h];
    for i in 0..h{
        for j in 0..w{
            grid[i][j] = s[i].chars().map(|x| x.to_digit(10))collect();
        }
    }
    }
