---
tags:
  - rust基礎
  - 文字列
  - char
  - to_digit
  - イテレータ
  - unwrap
  - map
problem: drill-a53
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a53|問題文]]

数字のみからなる文字列 S の各桁の和を出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans: u32 = s.chars().map(|c| c.to_digit(10).unwrap()).sum();
    println!("{}", ans);
}
```

最初は `.map(...).unwrap()` と書いて、イテレータ全体に `unwrap` をかけようとしてエラー。**`unwrap` はクロージャの中で1個ずつかける**のが正解。

---

## 学んだこと

### `char.to_digit(10)` — 数字文字を数値に変換

```rust
let d: Option<u32> = c.to_digit(10);
```

- `c` が `'0'..='9'` なら `Some(値)`、それ以外（`'a'`, `' '` など）なら `None`
- 引数 `10` は基数（16進数で扱いたいなら `to_digit(16)`、`'a'` も解釈される）
- 戻り値は **`Option<u32>`** — 失敗を表現できる

別解（`u8` 経由のテク）:
```rust
let d: i64 = (c as u8 - b'0') as i64;
```
`'0' = 0x30, '1' = 0x31, ...` の ASCII 連番を利用。Option を返さないので「数字以外」が来たら壊れた値になる（要注意）。

知識レベル: 🔵 説明可能

---

### `unwrap` はイテレータ全体じゃなく**個別の Option** に対して使う

```rust
// ❌ map の結果（Map 型）に unwrap → コンパイルエラー
s.chars().map(|c| c.to_digit(10)).unwrap()

// ✅ クロージャの中で各 Option を unwrap
s.chars().map(|c| c.to_digit(10).unwrap()).sum()
```

`unwrap` は `Option<T>` / `Result<T, E>` のメソッドで、**1個の値**から中身を取り出すもの。イテレータ（複数要素を流すもの）には効かない。

**判断方法**: その時点で扱っている型を確認する。
- `c.to_digit(10)` の型は `Option<u32>` → `unwrap()` 可
- `s.chars().map(...)` の型は `Map<...>`（イテレータ） → `unwrap()` 不可

知識レベル: 🔵 説明可能

---

### イテレータの「各要素処理」パターン整理

```rust
// 各要素を変換 → sum
.map(|c| c.to_digit(10).unwrap()).sum::<u32>()

// 各要素から Option を取り出す + None を除外
.filter_map(|c| c.to_digit(10)).sum::<u32>()

// 全部が Some なら Vec を取る、1つでも None なら None
.map(|c| c.to_digit(10)).collect::<Option<Vec<u32>>>()
```

特に **`filter_map`** は「`Option` を返す関数を `map` + `filter` した結果」を1ステップで作れる便利メソッド。「数字だけ拾う」みたいな場面で活躍。

知識レベル: 🟡 雰囲気理解

---

### `sum()` の型推論問題

```rust
let ans = s.chars().map(|c| c.to_digit(10).unwrap()).sum();   // ← 型不明
let ans: u32 = ...;                                            // OK
let ans = ...sum::<u32>();                                     // OK（ターボフィッシュ）
```

`sum` は収集先の型が決まらないとコンパイルできない。[[drill-a49]] と同じ罠。

知識レベル: 🟢 実装可能

---

### `Option` 操作早見表（追加分）

| やりたいこと | メソッド |
|---|---|
| 中身を取り出す（失敗で panic） | `unwrap()` |
| 中身を取り出す（None なら デフォルト） | `unwrap_or(d)` |
| 中身に関数適用 | `map(f)` |
| 中身に関数適用 + None ならデフォルト | `map_or(d, f)` |
| 中身に関数適用（関数も Option 返す） | `and_then(f)` |
| Option<Option<T>> を Option<T> にする | `flatten()` |
| イテレータ内の Some だけ集める | `filter_map(f)` |

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a52]] — `Option` ハンドリング（match / map_or / if let）
- [[drill-a49]] — `sum::<T>()` の型指定
- [[drill-a42]] — `char` の `is_ascii_*` / `to_ascii_*` 系
- [[drill-a39]] — `String` ↔ `Vec<char>`、UTF-8
