---
tags:
  - 貪欲法
  - ソート
  - イテレータ
  - enumerate
  - scan
  - sum
problem: drill-c2
date: 2026-06-22
difficulty: drill
---

## 問題

[[../../../contests/drill/c2|問題文]]

$N$ 個のジョブを1台で順に処理する。各ジョブの完了時刻（待ち時間）の合計を最小にする。

## 実装アプローチ

**認識トリガー**: 「完了時刻／待ち時間の合計を最小化」→ **処理時間の昇順（SPT: Shortest Processing Time）に並べる貪欲**。
合計は $10^{14}$ 級 → `i64`。

### 提出したコード（寄与の式）

```rust
t.sort();
let mut ans: i64 = 0;
for i in 0..n {
    ans += t[i] * (n as i64 - i as i64);
}
println!("{}", ans);
```

`t[i] * (n-i)` の意味：昇順に並べたとき `i` 番目（0-indexed）に小さいジョブの処理時間 `t[i]` は、
**自分自身とそれより後ろの全ジョブ**の完了時刻に乗る。その個数が `n - i`。
→ 「各 `t[i]` が何回数えられるか（寄与）」で総和を出している。正しいしムダがない。

## このセッションで学んだこと

### 同じ答えの2つの見方：「寄与」と「累積」

コメントが想定していた「累計(acc)と総和(sum)」は別の見方で、**同じ値を出す**。

```rust
let mut acc = 0;       // ここまでの完了時刻（=prefix sum）
let mut ans = 0;
for &x in &t {
    acc += x;          // このジョブの完了時刻
    ans += acc;        // それを総和に足す
}
```

- **寄与の見方**（提出版）: `t[i]` が何回足されるか = `n - i` 回 → `Σ t[i]*(n-i)`
- **累積の見方**（コメント版）: 完了時刻 = prefix sum、その総和 → `Σ (t[0]+…+t[i])`

両方とも正しい。どちらの視点も持てると、こういう「合計の最小化」系で詰まりにくい。

🟢 実装可能

---

### `for i in 0..n` + 添字 → `enumerate()`

添字と値の両方が欲しいときは `enumerate()` が `(i, &x)` を順に返してくれる。
`i` を自分で回さなくて済む。

```rust
let ans: i64 = t.iter()
    .enumerate()
    .map(|(i, &x)| x * (n - i) as i64)
    .sum();
```

→ [[../../topics/iterator-catalog|iterator-catalog]]

🟢 実装可能

---

### `scan`：状態を持ちながら map する

累積の見方をイテレータで書くと `scan` になる。`map` に「持ち越す状態」を足したもの。

```rust
let ans: i64 = t.iter()
    .scan(0i64, |acc, &x| { *acc += x; Some(*acc) })  // 各ステップの完了時刻を流す
    .sum();
```

`scan(初期状態, |状態, 要素| Some(出力))`。`Some(...)` を `None` にすると途中で打ち切れる。
便利だが少し玄人向け。競プロでは無理に使わなくてよい。

🟡 雰囲気理解

---

### キャストは `(n - i) as i64` で1回にまとめる

`n` も `i` も `usize`。`i < n` が保証されているので、先に usize のまま引いてから `as i64` でよい。

```rust
n as i64 - i as i64   // キャスト2回
(n - i) as i64        // キャスト1回・スッキリ（i<n なので桁あふれなし）
```

→ [[../../topics/integer-types-overflow|integer-types-overflow]]

🟢 実装可能

## この型の問題のテンプレ

「完了時刻／待ち時間の合計を最小化」と来たら **処理時間の昇順（SPT）にソート**。
答えは「各 `t[i]` の寄与 `(n-i)` 倍を足す」か「prefix sum の総和」のどちらでも出せる。
