---
tags:
  - 全探索
  - ラベル付きbreak
  - インデックスループ
  - 計算量
  - rust基礎
problem: b03
date: 2026-05-31
difficulty: tessoku-B
---

# B03 - Supermarket 1

[[../../tessoku-book/b03|問題文]]

N個の商品から**異なる3つ**を選んで合計がちょうど1000円になるか判定する。

## 実装アプローチ

### 提出コード（3重ループ + ラベル付き break）

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut ans = false;
    'outer: for i in 0..n-2 {
        for j in i+1..n-1 {
            for k in j+1..n {
                if a[i] + a[j] + a[k] == 1000 {
                    ans = true;
                    break 'outer;
                }
            }
        }
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
```

計算量: O(n³)。N ≤ 100 なので最大約161,700回で余裕。

### 改善案（2重ループ + HashSet で O(n²)）

i, j を固定して `1000 - a[i] - a[j]` を HashSet で O(1) 検索する。  
N が大きい場合に有効だが、「同じインデックスを使わない」処理が少し厄介。

---

## このセッションで学んだこと

### ラベル付き break

多重ループを一気に抜けるための Rust の機能。

```rust
'outer: for i in 0..n {
    for j in 0..n {
        if 条件 {
            break 'outer;  // 'outer のループまで一気に抜ける
        }
    }
}
```

- ラベルはシングルクォート `'` で始める（ライフタイムと同じ記法）
- バッククォート `` ` `` では**ない**（最初に間違えた）
- `break 'label` で指定したラベルのループを終了する

🟢 実装可能

---

### 「異なるN個」はインデックスで区別する

「異なる3つの商品」を値の大小で判定しようとすると、同じ値の商品が複数ある場合にバグる。

**NG（値の大小比較）：**
```rust
for &v1 in &a {
    for &v2 in &a {
        if v1 > v2 { // 値が同じだと同じ商品でも通過してしまう
```

**OK（インデックスで区別）：**
```rust
for i in 0..n-2 {
    for j in i+1..n-1 {
        for k in j+1..n {
```

`i < j < k` を強制することで、常に異なるインデックスの3商品を選ぶことができる。

反例: A = `[200, 600, 1]` の場合  
値比較だと i=j=0, k=1 → `a[0]+a[0]+a[1] = 1000` で誤って Yes になる。

🟢 実装可能

---

### ループ範囲の設計

3重ループで `i < j < k < n` を満たすための範囲：

```rust
for i in 0..n-2    // i は最大 n-3（j, k の分を残す）
for j in i+1..n-1  // j は最大 n-2（k の分を残す）
for k in j+1..n    // k は最大 n-1
```

`j in i..n-1`（`i+1` でなく `i` から始める）にすると `i == j` が発生し、同じ商品を2回使ってしまう。

🟢 実装可能
