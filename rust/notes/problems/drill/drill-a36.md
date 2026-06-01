---
tags:
  - rust基礎
  - スライス
  - reverse
  - 1-indexed
  - Vecメソッドまとめ
problem: drill-a36
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a36|問題文]]

数列 `(1, 2, ..., N)` の L 項目から R 項目までを逆順に並べ替えて空白区切りで出力する。ABC356-A と同じ問題。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }
    let mut a: Vec<usize> = (1..=n).collect();
    a[l-1..r].reverse();
    let s: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" "));
}
```

ポイントは `a[l-1..r].reverse()` の1行。スライスを切り出してその場で逆順にできる。

---

## 学んだこと

### スライスの `.reverse()` — その場で部分配列を逆順にする

```rust
let mut a: Vec<usize> = (1..=n).collect();
a[l-1..r].reverse();
```

- `a[l-1..r]` は `a` の一部分を指すスライス
- `.reverse()` を呼ぶと **元の `a` がその場で書き換わる**（in-place）
- `Vec` 全体だけでなく **スライス（部分配列）に対しても効く** のが強い

例: N=5, L=2, R=3
- `a = [1, 2, 3, 4, 5]`
- `a[1..3]` → `[2, 3]` の部分
- `.reverse()` → `a = [1, 3, 2, 4, 5]`

知識レベル: 🟢 実装可能

---

### `.reverse()` と `.iter().rev()` の違い

| やりたいこと | 使うもの | 元の配列 |
|---|---|---|
| その場で配列を逆順にする | `.reverse()` | 変わる |
| 逆順で読みたいだけ | `.iter().rev()` | 変わらない |

`.iter().rev()` は「逆順イテレータ」を返すだけ。表示用に逆順で見るならこっち、配列そのものを書き換えたいなら `.reverse()`。

知識レベル: 🔵 説明可能

---

### 1-indexed → 0-indexed の罠

問題文の「L項目」は 1始まり、配列インデックスは 0始まり。

- L項目 → インデックス `l-1`
- R項目 → インデックス `r-1`
- レンジ `..r` は **r を含まない** ので、R項目を含めるには `l-1..r` でちょうど良い

| 問題文 | スライス |
|---|---|
| L〜R 項目 | `a[l-1..r]` |
| 1〜N 項目（全部） | `a[0..n]` または `a[..]` |

知識レベル: 🟢 実装可能

---

### なぜ `map` が今回向かないか

`map` は「全要素に同じ変換」を適用するもの。今回は「位置によって扱いを変える（L〜R 内は逆順、外はそのまま）」なので `map` だと面倒。

**判断基準**:
- 全要素を同じルールで変換 → `map`
- 一部分だけ操作 → スライス記法 + `.reverse()` / `.sort()` など

知識レベル: 🟡 雰囲気理解

---

## A問題で出る「数列操作」チートシート

A問題レベルでよく出る Vec / スライス操作。標準ライブラリだけで1〜2行で済むものばかり。

### 反転・ソート
```rust
a.reverse();                   // 全体反転
a[l-1..r].reverse();           // 区間反転
a.sort();                      // 昇順
a.sort_by(|x, y| y.cmp(x));    // 降順
a[l-1..r].sort();              // 区間だけソート
```

### 集計
```rust
let sum: i64 = a.iter().sum();
let sum: i64 = a[l-1..r].iter().sum();        // 区間和
let mx = *a.iter().max().unwrap();
let mn = *a.iter().min().unwrap();
```

### 検索・カウント
```rust
let cnt = a.iter().filter(|&&x| x >= k).count();   // 条件を満たす個数
if a.contains(&x) { ... }                           // 含むか
let pos = a.iter().position(|&v| v == x);          // 最初の位置 Option<usize>
```

### 変換
```rust
let b: Vec<i64> = a.iter().map(|&x| x * 2).collect();   // 全要素変換
a.dedup();                                              // 重複削除（ソート済み前提）
a.rotate_left(k);                                       // 左にk個ローテート
a.rotate_right(k);
```

### 隣接要素
```rust
let diff: Vec<i64> = a.windows(2).map(|w| w[1] - w[0]).collect();
```

### 累積和（B問題以降頻出）
```rust
let mut s = vec![0i64; n + 1];
for i in 0..n {
    s[i+1] = s[i] + a[i];
}
// 区間 [l, r] の和は s[r] - s[l-1]
```

知識レベル: 🟡 雰囲気理解（一覧として把握、個別はその都度確認）

---

## 関連ノート

- [[drill-a35]] — 数列を作って空白区切りで出力（`(1..=n).collect()` + `join`）
- [[drill-a30]] — `filter` + `count` パターン
