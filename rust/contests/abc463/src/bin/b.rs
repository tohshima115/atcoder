use proconio::input;

fn main() {
    input! {
        n: usize,
        x: char,
        seat: [String; n],
    }
    let mut ans = false;
    let num:usize = if x == 'A' {
        0
    }else if x == 'B' {
        1
    }else if x == 'C' {
        2
    }else if x == 'D' {
        3
    }else {
        4
    };
    for s in &seat{
        let c = s.chars().nth(num).unwrap();
        if c == 'o' {
            ans = true;
            break;
        }
    }
    println!("{}", if ans {"Yes"} else {
        "No"
    });
}
