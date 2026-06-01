---
tags:
  - rust基礎
  - イテレータ
  - filter
  - 参照
problem: drill-a30
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a30|問題文]]

N個の整数からK以上の要素の個数を出力する。

## 実装

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let count: usize = a.iter().filter(|&&x| x >= k).count();
    println!("{}", count);
}
```

**元のコード（全探索）も正解**:

```rust
let mut count: i64 = 0;
for &v in &a {
    if v >= k {
        count += 1;
    }
}
```

N≤100 なので全探索で十分。`filter` + `count` はワンライナー版。

---

## 学んだこと

### `filter` + `count` — 条件を満たす個数を数える

```rust
a.iter().filter(|&&x| x >= k).count()
```

- `filter` で条件を満たす要素だけ通す
- `count()` で個数を数える（`usize` を返す）

「条件を満たす個数」系の問題はこのパターン一択。

知識レベル: 🟢 実装可能

---

### `filter` のクロージャは `&&x`（参照が二重になる理由）

```rust
a.iter()          // &i64 のイテレータ
.filter(|x| ...)  // x は &&i64（filterがもう一枚&を足す）
```

`iter()` が `&i64` を返し、`filter` のクロージャがさらに `&` を1枚足すので `&&i64` になる。`&&x` でそれを2回外す。

```rust
.filter(|&&x| x >= k)   // 呪文として覚える
.filter(|&x| *x >= k)   // 同じ意味
```

**今は呪文として丸暗記でOK**。参照・借用はRustで一番難しい部分。

| クロージャ | 使う場面 |
|---|---|
| `\|&&x\|` | `filter` |
| `\|&x\|` | `map` |

知識レベル: 🟡 雰囲気理解
