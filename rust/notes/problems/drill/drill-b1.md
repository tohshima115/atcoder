---
tags:
  - rust基礎
  - イテレータ
  - filter
  - count
  - 参照
  - XOR
problem: drill-b1
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/b1|問題文]]

$N$ 個の電球（最初は全消灯）に対し、$Q$ 回のトグル操作を行う。全操作後に点灯している電球の個数を出力する。

## 実装アプローチ

**提出コード**:
```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }
    let mut light: Vec<bool> = vec![false; n];
    for i in 0..q {
        light[t[i] - 1] ^= true;
    }
    let ans = light.iter().filter(|&&x| x).count();
    println!("{}", ans);
}
```

制約が `N, Q ≤ 1000` と小さいので、bool 配列をそのままトグルして最後に数える素直な O(N+Q) でOK。

**もう一歩（イテレータで回す）**:
```rust
for &x in &t {
    light[x - 1] ^= true;
}
```
`for i in 0..q` + `t[i]` のインデックスアクセスより、`for &x in &t` で要素を直接回す方が off-by-one が起きにくく Rust らしい。`q` の長さを意識しなくて済む。

---

## 学んだこと

### ON/OFF のトグルは `^= true`（XOR代入）

```rust
light[t[i] - 1] ^= true;
```

`bool ^ true` は値を反転する（`false ^ true = true`, `true ^ true = false`）。
`if light[x] { light[x] = false } else { light[x] = true }` と書かずに1行で済む。スイッチの反転＝XOR、という対応を覚えておくと便利。

知識レベル: 🟢 実装可能

---

### `filter(|&&x| x)` の `&&` は「運」じゃなく参照の枚数

bool の `Vec` から「true の個数」を数えるイディオム。`&&` が2枚必要な理由を型で追うと：

```rust
light.iter()        // 要素の型は &bool（参照を返す）
.filter(|x| ...)    // filter は要素のさらに参照を渡す → x: &&bool
```

`iter()` の要素が `&bool`、`filter` はそれを消費せず「見るだけ」なのでさらに `&` を付けて渡す → クロージャ引数は `&&bool`。

**剥がし方は3通り（全部正しい）**:
```rust
.filter(|x| **x)     // x: &&bool → ** で2枚剥がす
.filter(|&x| *x)     // 引数で1枚 → x:&bool → * でもう1枚
.filter(|&&x| x)     // 引数で2枚剥がして x:bool そのまま（採用）
```

エラー `expected bool, found &bool` が出たら「`&` か `*` を1枚足す」で狙って直せる。`filter` のクロージャ戻り値は厳密に `bool` を要求するので、自動デリファレンスに頼らず剥がす。

知識レベル: 🔵 説明可能

---

### `iter()` の `|&&x|` と `chars()` の `|&ch|` の対比

| 元 | イテレータ要素 | filter のクロージャ |
|---|---|---|
| `Vec<bool>.iter()` | `&bool` | `|&&x|`（2枚） |
| `String.chars()` | `char`（値） | `|&ch|`（1枚） |

`chars()` は `char` を**値**で返すので1枚で済む。`iter()` は**参照**を返すので filter で2枚になる。「元のイテレータが値を返すか参照を返すか」で枚数が変わる。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-a54]] — `chars().filter(|&ch| ...).count()`（こちらは1枚、対比）
- [[drill-a30]] — `filter` で `|&&x|`（同じ `iter()` パターン）
