use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }
    fn jan_num(x: String) {
        match x {
            'rock' => -1,
            'scissors' => 0,
            _=> 1,
        }
    }
    let a_num = jan_num(a);
    let b_num = jan_num(b);
    let ans = if a_num == b_num {
        "Draw"
    }else if a_num - b_num == -1 {
        ""
    }
}
