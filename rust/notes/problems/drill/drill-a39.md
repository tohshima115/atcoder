---
tags:
  - rust基礎
  - 文字列
  - Vec<char>
  - UTF-8
  - chars
problem: drill-a39
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a39|問題文]]

英小文字のみの文字列 S の K 文字目を文字 C に置き換えて出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        s: String,
        k: usize,
        c: char,
    }
    let mut s_vec: Vec<char> = s.chars().collect();
    s_vec[k - 1] = c;
    let ans: String = s_vec.iter().collect();
    println!("{}", ans);
}
```

ポイント: **String は直接書き換えできない**ので `Vec<char>` に変換 → 書き換え → `String` に戻す、の3ステップ。

---

## 学んだこと

### `String` は直接インデックスで書き換えできない

```rust
let mut s = String::from("abcde");
s[2] = 'x';   // ← コンパイルエラー
```

理由: Rust の `String` は **UTF-8** で保持されている。1文字が1バイトとは限らない（ASCII は1バイト、日本語は3バイト等）ので、`s[2]` が「3文字目の先頭バイト」なのか「ある文字の途中」なのか判定できない。安全のため Rust は `String` への直接インデックスをそもそも禁止している。

→ 1文字単位で扱いたいときは `Vec<char>` を経由するのが定石。

知識レベル: 🔵 説明可能

---

### `String` ↔ `Vec<char>` の往復

**String → Vec<char>**:
```rust
let s_vec: Vec<char> = s.chars().collect();
```
`s.chars()` は1文字ずつの `char` を返すイテレータ。

**Vec<char> → String**:
```rust
let ans: String = s_vec.iter().collect();
```
`char` のイテレータは `String` に直接 `collect` できる（`FromIterator<char> for String` が実装されている）。

**ハマりポイント**: `join` は `Vec<char>` には使えない（`join` は文字列スライス用）。`collect::<String>()` を使う。

知識レベル: 🟢 実装可能

---

### `Vec<char>` は普通にインデックス代入できる

```rust
let mut s_vec: Vec<char> = s.chars().collect();
s_vec[k - 1] = c;   // OK
```

`Vec<char>` は固定サイズの `char` の配列なので、各要素は4バイト固定（`char` は Unicode スカラー値）。だからインデックスアクセス・代入が安全にできる。

→ `remove` + `insert` で書き換える必要はない（1要素差し替えなら `[i] = x` で十分）。

知識レベル: 🟢 実装可能

---

### proconio の `c: char` で1文字受け取り

```rust
input! {
    c: char,
}
```

入力に1文字だけ来るときは `char` で受け取れる。`String` で受け取って `.chars().next().unwrap()` する必要はない。

知識レベル: 🟢 実装可能

---

## 文字列操作の定石（drill 範囲）

| やりたいこと | やり方 |
|---|---|
| K 文字目を書き換え | `Vec<char>` 経由で `[k-1] = c` |
| 1文字ずつ走査 | `s.chars()` でイテレート |
| 長さ（文字数） | `s.chars().count()` または `s.len()`（バイト数）に注意 |
| `Vec<char>` → `String` | `.iter().collect::<String>()` |
| 部分文字列 | `&s[l..r]`（ただしバイト境界に注意。ASCII限定なら安全） |

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a38]] — `insert`（Vec の挿入。文字列だと使えない罠と対比）
- [[drill-a35]] — `Vec<String>` を `join` で連結（chars だと使えない、と対比）
