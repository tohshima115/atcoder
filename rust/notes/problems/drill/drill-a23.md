---
tags:
  - rust基礎
  - イテレータ
  - クロージャ
  - 文字列変換
  - 進数変換
  - Result型
problem: drill-a23
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a23|問題文]]

非負整数 N の各桁の和を出力する。

## 実装

```rust
use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let ans = n.to_string()
               .chars()
               .map(|c| c.to_digit(10).unwrap() as i64)
               .sum::<i64>();
    println!("{}", ans);
}
```

最初は上位桁から順に引いていくゴリ押しで解いた。「文字列に変換して1桁ずつ取り出す」方が素直。

---

## 学んだこと

### メソッドチェーンは左から右に型が変わっていく

```rust
n.to_string()   // i64 → String
 .chars()        // String → Iterator<char>
 .map(...)       // Iterator<char> → Iterator<i64>
 .sum::<i64>()   // Iterator<i64> → i64
```

各ステップで型が変わるイメージ。左から右に処理が流れる。

知識レベル: 🟢 実装可能

---

### `.chars()` は `char` のイテレータを返す（数値ではない）

`.chars()` で1文字ずつに分解できるが、取り出せるのは `char`（文字）であり数値ではない。

```rust
'1' + '2'  // コンパイルエラー。char は足し算できない
```

`.map()` で `char → i64` に変換してから `.sum()` する必要がある。

知識レベル: 🔵 説明可能

---

### クロージャ `|引数| 処理` — その場で定義する使い捨て関数

```rust
.map(|c| c.to_digit(10).unwrap() as i64)
//   ^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   引数  処理（charをi64に変換）
```

`|c|` の `c` は自分でつけた名前。関数で書くと同じ意味になる：

```rust
fn convert(c: char) -> i64 {
    c.to_digit(10).unwrap() as i64
}
.map(convert)  // 同じ
```

`map` や `filter` はクロージャを受け取って各要素に適用する。

知識レベル: 🟡 雰囲気理解

---

### `to_digit(基数)` — 文字を数値に変換

引数は基数（何進数として解釈するか）。

```rust
'7'.to_digit(10)  // Some(7)  ← 10進数で有効
'7'.to_digit(2)   // None     ← 2進数では無効（0か1しかない）
'1'.to_digit(2)   // Some(1)
```

`Option<u32>` を返すので `.unwrap()` で取り出す。

知識レベル: 🟢 実装可能

---

### 進数変換

```rust
// 10進数 → 2進数文字列
format!("{:b}", 13)     // "1101"
format!("{:o}", 8)      // "10"（8進数）
format!("{:x}", 255)    // "ff"（16進数）

// 2進数文字列 → 10進数
i64::from_str_radix("1101", 2).unwrap()  // 13
```

`from_str_radix` は失敗しうる（不正な文字列の可能性）ので `Result` で返る。制約上正しいとわかっている場合は `.unwrap()` で取り出す。

知識レベル: 🟡 雰囲気理解

---

### Option と Result — 2種類の箱

| 箱 | 中身 | 使われる場面 |
|---|---|---|
| `Option<T>` | `Some(値)` / `None` | 値があるかないか |
| `Result<T, E>` | `Ok(値)` / `Err(エラー)` | 成功か失敗か |

競プロではどちらも `.unwrap()` で開ければOK。現場では panic を避けるため別の書き方をする（[[drill-a21]] 参照）。

知識レベル: 🟡 雰囲気理解
