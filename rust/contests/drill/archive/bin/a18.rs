use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }
    fn jan_num(x: &str) -> i32 {
        match x {
            "rock" => 0,
            "scissors" => 1,
            "paper" => 2,
            _=> unreachable!(),
        }
    }
    let a_num = jan_num(&a);
    let b_num = jan_num(&b);
    let ans = if a_num == b_num {
        "Draw"
    }else if (a_num - b_num + 3) % 3 == 2 {
        "Takahashi"
    }else{
        "Aoki"
    };
    println!("{}", ans);
}
