---
tags:
  - rust基礎
  - イテレータ
  - Option型
  - 文字列変換
problem: drill-a24
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a24|問題文]]

正整数 N の各桁のうち最大の値を出力する。

## 実装

```rust
use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let ans: i64 = n.to_string()
                    .chars()
                    .map(|c: char| c.to_digit(10).unwrap() as i64)
                    .max()
                    .unwrap();
    println!("{}", ans);
}
```

a23（各桁の和）と同じ構造で `.sum()` を `.max()` に変えるだけ。

**最初の失敗**: `.max()` の後の `.unwrap()` を忘れた。`Option<i64>` を `println!` に渡してコンパイルエラー。

---

## 学んだこと

### `.max()` は `Option` を返す、`.sum()` は返さない

```rust
v.iter().sum::<i64>()  // i64（空でも0が返せる）
v.iter().max()         // Option<i64>（空なら最大値が定義できない）
```

空のイテレータに `.max()` を呼んだら最大値が存在しないため `Option` で包まれる。`.sum()` は空なら 0 を返せるので `Option` にならない。

知識レベル: 🔵 説明可能

---

### コンパイルエラーから `Option` に気づく

```
`Option<i64>` cannot be formatted with the default formatter
```

エラーに `Option<...>` が出てきたら `.unwrap()` が必要のサイン。

知識レベル: 🟢 実装可能
