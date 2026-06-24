// C-4: 荷物の分割で最大負荷を最小に（答えで二分探索）
// 認識トリガー: 「最大を最小化」+「ある値で可能か?の判定が簡単」→ 答えで二分探索
// 判定: 容量 x で左から貪欲に詰めて使うトラック数 <= K か
// lo = max(w), hi = sum(w) から while lo < hi

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        w: [i64; n],
    }
    fn ok(cost:i64) {
        let mut weight: i64 = 0;
        let mut count: i64 = 1;
        for i in 0..n {
            if weight + w[i] <= cost {
                weight += w[i];
            }else{
                weight = 0;
                count += 1;
            }
        }
        return;
        count <= k;
    }
    let mut lo = *w.iter().max().unwrap(); // 1個でも積めないとダメなので下限はmax
    let mut hi = w.iter().sum::<i64>();    // 全部1台に積む
    while lo < hi {
        let mid = (lo + hi) / 2;
        if ok(mid) { hi = mid; } else { lo = mid + 1; }
    }
    println!("{}", lo);
}
