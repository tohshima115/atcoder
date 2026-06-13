---
tags:
  - rust基礎
  - イテレータ
  - filter
  - collect
  - cct
  - 落とし穴
problem: abc462a
date: 2026-06-13
difficulty: abc-A
---

## 問題へのリンク

[ABC462 A - Secret Numbers](https://atcoder.jp/contests/abc462/tasks/abc462_a)

## 問題の要約

英小文字と数字からなる文字列 S から、数字だけを順序を保って取り出した文字列を出力する。数字が無ければ空文字列。

## 実装アプローチ（理想的・1分で書けた）

```rust
use proconio::input;

fn main() {
    input! { s: String }
    let ans: String = s.chars().filter(|x| x.is_ascii_digit()).collect();
    println!("{}", ans);
}
```

- `chars().filter(...).collect::<String>()` で「条件を満たす文字だけ集めて文字列化」。
- `is_ascii_digit()` は `char` のメソッドで `'0'..='9'` 判定。直すところゼロ。

## このセッションで学んだこと

### コードは正しいのに `cct` が WA を出す「偽WA」がある（本題）

このA、**コードは100点なのに `cct a` で WA が出て20分溶かした**。原因はローカルテスタ（cargo-compete）側:

- サンプル `codequeen` の期待出力は**空文字列（0行）**。
- `println!` は末尾に `\n` を足すので、自分の出力は「**空行が1つ**（1行）」。
- yml の `match: Lines`（行単位の厳密比較）が「1行 ≠ 0行」で WA を出していた。

`ans = ""` にしても `println!` が `\n` を足すので同じく落ちる。

**直し方はテスタが `note:` で教えてくれていた**:

```
note:
whitespace-separated words matched. try setting `match` to `SplitWhitespace`
```

yml の `match: Lines` → `match: SplitWhitespace` で 4/4 Accepted。
（本番 AtCoder ジャッジは末尾改行を無視するので、提出は最初から AC だった。）

教訓 = **WA が出たら `note:` 行を必ず読む**。詳細は [[cargo-compete-test-pitfalls]]。

知識レベル: 🔵 説明可能

---

### `filter().collect::<String>()` で文字列フィルタ

文字列から条件を満たす文字だけ抜き出す定番。`collect` の集める先の型は左辺の型注釈（`let ans: String`）かターボフィッシュで決める。([[iterator-catalog]] / [[string-char-conversion]])

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[cargo-compete-test-pitfalls]] — cct 偽WA の落とし穴集（本題はこっち）
- [[abc462b]] · [[abc462c]] · [[abc462d]] — 同じコンテスト
- [[string-char-conversion]] · [[iterator-catalog]]
