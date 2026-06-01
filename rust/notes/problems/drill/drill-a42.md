---
tags:
  - rust基礎
  - 文字列
  - char
  - map
  - ascii
  - if式
problem: drill-a42
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a42|問題文]]

英大文字・英小文字からなる文字列 S の大文字と小文字を入れ替えて出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans: String = s.chars().map(|x| {
        if x.is_ascii_uppercase() {
            x.to_ascii_lowercase()
        } else {
            x.to_ascii_uppercase()
        }
    }).collect();
    println!("{}", ans);
}
```

`s.chars()` → `map` で1文字ずつ変換 → `collect::<String>()` の定型パターンに乗せられた。

---

## 学んだこと

### `char` の大文字小文字判定・変換 API

| メソッド | 内容 |
|---|---|
| `c.is_ascii_uppercase()` | ASCII 大文字か |
| `c.is_ascii_lowercase()` | ASCII 小文字か |
| `c.to_ascii_uppercase()` | ASCII 範囲で大文字化（他はそのまま） |
| `c.to_ascii_lowercase()` | ASCII 範囲で小文字化（他はそのまま） |
| `c.is_uppercase()` / `is_lowercase()` | Unicode 全体で判定 |
| `c.to_uppercase()` / `to_lowercase()` | Unicode 対応（複数文字を返すイテレータになる） |

**競プロでは `is_ascii_*` / `to_ascii_*` を選ぶのが正解**:
- 速い（単純な範囲チェック）
- 戻り値が `char` 1個で扱いやすい（Unicode 版は `to_uppercase()` がイテレータを返すので扱いが面倒）
- AtCoder の問題は英大小文字限定が多い

知識レベル: 🟢 実装可能

---

### `map` の中で `if` を式として使う

```rust
let ans: String = s.chars().map(|x| {
    if x.is_ascii_uppercase() {
        x.to_ascii_lowercase()
    } else {
        x.to_ascii_uppercase()
    }
}).collect();
```

- Rust の `if` は **式**（値を返す）。`;` を付けないことで「値を返す」ことになる
- クロージャの最後の式がそのまま戻り値
- `match` でも同じ書き方ができる

知識レベル: 🟢 実装可能

---

### `chars` → `map` → `collect::<String>` のパターン

文字列を「1文字ずつ変換して新しい文字列を作る」ときのテンプレ:

```rust
let ans: String = s.chars().map(|c| /* 変換 */).collect();
```

- `s.chars()` は値で返してくれるので `.copied()` 不要（[[drill-a41]] で学んだやつ）
- `char` のイテレータは `String` に直接 `collect` できる

知識レベル: 🟢 実装可能

---

### ネタ: ASCII の大文字↔小文字は 0x20 ビット違い

```rust
let ans: String = s.chars().map(|c| (c as u8 ^ 0x20) as char).collect();
```

- `'A' = 0x41`, `'a' = 0x61` → 差は `0x20`
- XOR で 0x20 ビットを反転すれば大↔小が入れ替わる
- **可読性は下がる** ので普段は使わない。雑学として知っとくと面白い

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a39]] — `Vec<char>` 経由で文字列操作
- [[drill-a41]] — `chars()` が値を返すので `.copied()` 不要
- [[drill-a30]] — `filter` の `|&&x|` パターン
