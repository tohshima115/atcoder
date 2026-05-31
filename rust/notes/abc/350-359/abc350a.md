---
tags:
  - rust基礎
  - 文字列スライス
  - parse
problem: abc350a
date: 2026-05-31
difficulty: abc-A
---

## 問題へのリンク

[ABC350 A - Past ABCs](https://atcoder.jp/contests/abc350/tasks/abc350_a)

## 問題の要約

長さ6の文字列 `S`（先頭3文字は `ABC`、末尾3文字は数字）が与えられる。`ABC001`〜`ABC349` のうち `ABC316` を除いた348個のいずれかなら `Yes`、そうでなければ `No`。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = false;
    let num: i64 = s[3..].parse().unwrap();
    if num >= 1 && num <= 349 && num != 316 {
        ans = true
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
```

末尾3文字の数字部分を `i64` に変換し、範囲チェックと除外条件で判定する。

## このセッションで学んだこと

### 文字列スライス `s[start..end]`

Rust の文字列はバイト列として管理されており、`s[3..]` でインデックス3以降の部分文字列（`&str`）を取り出せる。

```rust
let s = String::from("ABC349");
let tail = &s[3..];   // "349"
let tail2 = &s[3..6]; // "349"（長さ6固定前提）
```

| 書き方 | 意味 |
|---|---|
| `s[3..]` | 4文字目以降すべて |
| `s[3..6]` | インデックス3〜5（長さ6固定前提の末尾3文字） |
| `s[s.len()-3..]` | 長さ不定でも末尾3文字 |

今回は長さ6固定なので `s[3..]` で4文字目以降 = 末尾3文字が取れる。

知識レベル: 🟢 実装可能

---

### 文字列 → 数値変換 `.parse()`

`&str` に `.parse::<T>()` を呼ぶことで数値型に変換できる。型注釈があれば型パラメータは省略可能。

```rust
let num: i64 = s[3..].parse().unwrap();
// または
let num = s[3..].parse::<i64>().unwrap();
```

- `.parse()` は `Result<T, E>` を返す
- 制約で入力形式が保証されている場合は `.unwrap()` でOK
- `to_num` というメソッドは存在しない（`.parse()` を使う）

知識レベル: 🟢 実装可能
