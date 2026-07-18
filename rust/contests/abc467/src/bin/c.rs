use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: i64,
        seta: [i64;n],
        setb: [i64;n-1],
    }
    let mut setx: Vec<i64> = vec![];
    for i in 0..n-1 {
        setx.push((seta[i] + seta[i+1]) % 2);
    }
    let mut p:Vec<usize> = vec![];
    for i in 0..n-1 {
        if setx[i] == 1 {
            p.push(i);
        }
    }
    let mut ans_a: i64 = 0;
    let mut ans_b: i64 = 0;
    if p.len() % 2 == 0 {
        let p_a = p;
        let p_b =pの前に0後ろにlen
    }else {
        let p_a = pの前に０
        let p_b =pの後ろにlen
    }
    それぞれ前から順番にペアとして間隔を足していってans_aとans_bで小さい方が正解
}
