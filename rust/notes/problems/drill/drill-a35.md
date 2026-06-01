---
tags:
  - rust基礎
  - イテレータ
  - 空白区切り出力
  - join
  - collect
problem: drill-a35
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a35|問題文]]

正整数 N が与えられる。数列 `(1, 2, ..., N)` を空白区切りで1行に出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let vec: Vec<usize> = (1..=n).collect();
    let s: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" "));
}
```

**ワンライナー版（itertools 使用）**:

```rust
use itertools::Itertools;
println!("{}", (1..=n).join(" "));
```

競プロでは itertools の `join` を脳死で使えるのが一番ラク。

---

## 学んだこと

### `(1..=n).collect::<Vec<usize>>()` — レンジから Vec を作る

```rust
let v: Vec<usize> = (1..=n).collect();      // [1, 2, ..., n]
let v: Vec<i64>   = (1..=n).map(|x| x as i64).collect();
let v = (1..=n).collect::<Vec<usize>>();    // ターボフィッシュ版
```

- `(1..=n)` は n を含む（`..=` は inclusive）
- `(1..n)` は n を含まない（`..` は exclusive）
- `.collect()` の収集先は型注釈かターボフィッシュ `::<>` で指定する

知識レベル: 🟢 実装可能

---

### 空白区切り出力の3パターン

**① itertools の join（おすすめ）**
```rust
use itertools::Itertools;
println!("{}", (1..=n).join(" "));
```
レンジに直接 `.join` できる。Vec を作らなくていい。

**② 標準ライブラリ（Vec<String> に変換してから join）**
```rust
let s: Vec<String> = vec.iter().map(|x| x.to_string()).collect();
println!("{}", s.join(" "));
```

**③ ループで手書き**
```rust
let mut out = String::new();
for i in 1..=n {
    if i > 1 { out.push(' '); }
    out.push_str(&i.to_string());
}
println!("{}", out);
```

**ハマりポイント**:
- `println!("{} ", i)` をループで書くと末尾に余計な空白が付いてWAになりうる
- `Vec<i64>` のままだと `.join(" ")` は使えない（`join` は文字列スライス用）

知識レベル: 🟢 実装可能

---

### `join` の挙動

`Vec<String>` や `&[String]` の要素を、間に区切り文字を挟んで1本の文字列に連結する。

```rust
let s = vec!["1".to_string(), "2".to_string(), "3".to_string()];
s.join(" ")   // "1 2 3"
s.join(",")   // "1,2,3"
s.join(" / ") // "1 / 2 / 3"
s.join("")    // "123"
```

- 区切り文字は **要素の間にだけ** 入る（先頭・末尾には付かない）
- だから「末尾の余計な空白問題」が自動で回避できる

知識レベル: 🔵 説明可能

---

### `map` のクロージャは `|x|` でOK（`|&x|` でも動く）

```rust
vec.iter().map(|x| x.to_string())
```

- `vec.iter()` が返すのは `&usize`
- 本来クロージャの引数の型は `&usize` だが、`.to_string()` は **自動参照外し（auto-deref）** で参照ごしでも呼べる
- だから `|x| x.to_string()` で動く
- `|&x| x.to_string()` も等価（パターンマッチで `&` を外して `x: usize` にする）

| クロージャ | 主な使う場面 |
|---|---|
| `\|&&x\|` | `filter`（filter が `&` を1枚足すので二重になる） |
| `\|&x\|`  | `map`（参照を外して値として扱う） |
| `\|x\|`   | `map`（参照のまま `.to_string()` 等のメソッド呼ぶ） |

[[drill-a30]] の `filter` の `|&&x|` と対比して覚えると整理しやすい。

知識レベル: 🟡 雰囲気理解
