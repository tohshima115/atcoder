use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [(i64,i64,i64,i64,i64,i64,i64,i64);t],
    }
    for (ax, ay, bx, by, cx, cy, dx, dy) in case {
        let mut ans = true;
        if (ax - bx) * (cy - dy) == (ay - by) * (cx - dx) {
            ans = false;
        }
        if (ay - by) * ((ay + by) - (cy + dy)) == (ax - bx) * ((cx + dx) - (ax + bx)) {
            ans = true;
        }
        println!("{}", if ans {"Yes"} else {"No"});
    }
}
