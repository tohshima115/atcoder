---
tags:
  - rust基礎
  - 回文判定
  - イテレータ
  - 参照外し
  - copied
  - chars
problem: drill-a41
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a41|問題文]]

英小文字のみの文字列 S が回文なら `Yes`、そうでなければ `No` を出力する。

## 実装アプローチ

**最短解（String のまま比較）**:
```rust
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let rev: String = s.chars().rev().collect();
    println!("{}", if s == rev { "Yes" } else { "No" });
}
```

**Vec<char> 経由版**:
```rust
let c: Vec<char> = s.chars().collect();
let c_rev: Vec<char> = c.iter().rev().copied().collect();
println!("{}", if c == c_rev { "Yes" } else { "No" });
```

最初に書いたコードでは `let c_rev = c.reverse();` としていた → `reverse()` は `()` を返すのでコンパイルエラー、かつ呼んだ時点で `c` 自身が逆順に書き換わるので二重にダメ。

---

## 学んだこと

### `reverse()` は破壊的、戻り値は `()`

```rust
let mut c: Vec<char> = ...;
let c_rev = c.reverse();   // c_rev は () 型、c 自身が逆順に変わる
```

「新しい逆順の Vec が欲しい」なら `.iter().rev().copied().collect()`、もしくは `.into_iter().rev().collect()`。

| メソッド | 戻り値 | 元の配列 |
|---|---|---|
| `a.reverse()` | `()` | 書き換わる |
| `a.iter().rev()` | 逆順イテレータ | 変わらない |
| `s.chars().rev()` | 逆順 char イテレータ | 変わらない |

知識レベル: 🔵 説明可能

---

### `String` 同士は `==` で直接比較できる

```rust
let s = String::from("abc");
let t = String::from("abc");
if s == t { ... }   // OK
```

回文判定は「元と逆順版を比較」だけで済む。`Vec<char>` 経由する必要はない。

知識レベル: 🟢 実装可能

---

### `iter()` と `chars()` の決定的な違い — 参照 vs 値

| メソッド | 要素の型 | 元 |
|---|---|---|
| `vec.iter()` | `&T`（参照） | 借りる |
| `vec.into_iter()` | `T`（値） | 消費 |
| `s.chars()` | `char`（値） | 借りる（生成系イテレータ） |

`s.chars()` は **UTF-8 バイト列をデコードして `char` を生成する** イテレータなので、最初から値で返してくれる。だから `.copied()` 不要で `.collect::<String>()` できる。

`vec.iter()` は単に Vec の中の要素を借りるだけなので `&T` を返す。値が必要な場面では `.copied()` で外す必要がある。

```rust
// chars: そのまま
let rev: String = s.chars().rev().collect();

// iter: copied が要る
let rev: String = v.iter().rev().copied().collect();

// into_iter: 消費するから値で取れる
let rev: String = v.into_iter().rev().collect();
```

知識レベル: 🔵 説明可能

---

### `.copied()` / `.cloned()` / `.map(|&x| x)` の使い分け

| 書き方 | いつ使う |
|---|---|
| `.copied()` | 中身が `Copy` な型（`i64`, `char`, `bool`, `usize`...） |
| `.cloned()` | `Copy` じゃないけど `Clone` できる型（`String`, `Vec`...） |
| `.map(\|&x\| x)` / `.map(\|x\| *x)` | 手書きしたいとき（中身は同じ） |

競プロで扱うのはほぼ `Copy` 型なので **`.copied()` 一択**。`|&&x|` 地獄から抜け出す道具。

```rust
// これは全部同じ意味
.copied()
.map(|x| *x)
.map(|&x| x)
```

**ハマりポイント**: `*v` は1個の参照外し演算子なので、`*v.iter().rev().collect()` のようにイテレータ全体に付けても効かない。要素ごとに `.map(|x| *x)` する必要がある。

知識レベル: 🔵 説明可能

---

### `if` は式 — `println!` の中に直接書ける

```rust
println!("{}", if ans { "Yes" } else { "No" });
```

Rust の `if` は式（値を返す）。`Yes`/`No` 判定の出力でよく使うパターン。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-a36]] — `reverse()` がその場で書き換える話
- [[drill-a39]] — `String ↔ Vec<char>` の往復、UTF-8 の話
- [[drill-a30]] — `|&&x|` で参照を二重に外すパターン（`.copied()` を使えばこれを避けられる）
