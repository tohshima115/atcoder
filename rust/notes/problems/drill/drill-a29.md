---
tags:
  - rust基礎
  - ソート
  - イテレータ
  - スライス
problem: drill-a29
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a29|問題文]]

N個の整数から大きい順に上位K個の合計を出力する。

## 実装

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }
    a.sort();
    let ans: i64 = a[(a.len() - k)..].iter().sum::<i64>();
    println!("{}", ans);
}
```

昇順ソートして後ろからk個をスライスで取り出して合計。

**別解（問題文に素直）**:

```rust
a.sort_by(|x, y| y.cmp(x));  // 降順
let ans: i64 = a.iter().take(k).sum();
```

---

## 学んだこと

### 降順ソートは呪文として覚える

```rust
a.sort();                     // 昇順（デフォルト）
a.sort_by(|x, y| y.cmp(x));  // 降順（呪文）
```

`x` と `y` を入れ替えると昇順、そのままだと降順。`sort()` は `()` を返すので `.rev()` はチェーンできない。

```rust
a.sort().rev()   // NG: sort()は()を返す
a.iter().rev()   // OK: イテレータにしてからrev()
```

知識レベル: 🟢 実装可能（呪文として）

---

### 後ろからk個を取り出す方法

```rust
// スライス（インデックスが明確なとき）
a[(n - k)..].iter().sum()

// rev() + take()（ソート済みで後ろからk個）
a.iter().rev().take(k).sum()
```

どちらも等価。スライスはインデックス計算が必要、`.rev().take(k)` はイテレータチェーンで書ける。

知識レベル: 🟢 実装可能

---

### `.take(k)` — イテレータの先頭k個を取る

```rust
a.iter().take(3)  // 先頭3個だけのイテレータ
```

降順ソート後に `.take(k)` で「上位k個」が取れる。`.skip(k)` と対になるメソッド。

知識レベル: 🟢 実装可能
