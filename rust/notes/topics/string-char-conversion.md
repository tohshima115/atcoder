---
tags:
  - topic
  - 文字列
  - char
  - UTF-8
  - to_digit
type: topic
date: 2026-06-01
---

# 文字列 ↔ char ↔ 数値 の変換

> **繰り返し課題**: `String` を1文字ずつ書き換えられない、`char` を数値にできない、スライスで panic、`join` が使えない…文字列まわりで毎回つまずく。
> 出てきた問題: [[drill-a23]] [[drill-a39]] [[drill-a41]] [[drill-a42]] [[drill-a46]] [[drill-a53]] [[drill-a54]]

## 大原則: `String` は UTF-8 なので直接いじれない

```rust
let mut s = String::from("abcde");
s[2] = 'x';          // ❌ コンパイルエラー（インデックス代入禁止）
```

1文字が1バイトとは限らない（ASCII=1, 日本語=3…）ため。1文字単位で扱うなら **`Vec<char>` を経由**する（[[drill-a39]]）。

```rust
let mut v: Vec<char> = s.chars().collect();  // String → Vec<char>
v[k-1] = c;                                   // これは安全に代入できる
let s: String = v.iter().collect();           // Vec<char> → String
```

## char ⇄ 数値

```rust
c.to_digit(10)            // '7' → Some(7)。Option<u32>（[[drill-a23]] [[drill-a53]]）
(c as u8 - b'0') as i64   // ASCII連番テク。Option を返さない＝危険も（[[drill-a53]]）
c.to_ascii_uppercase()    // 大文字化（[[drill-a42]]）
c.is_ascii_uppercase()    // 判定。競プロは ascii 版を選ぶ（速い・char1個）
```

`unwrap` は `to_digit` の **直後**に。`map` 全体には付かない → [[option-result-handling]]。

## 文字列化: `join` か `collect::<String>` か ← 頻出ミス

| 元の型 | 文字列化 |
|---|---|
| `Vec<String>` / `&[&str]` | `.join(" ")` （区切りは要素**間**だけ＝末尾空白問題が出ない、[[drill-a35]]） |
| `Vec<char>` / `chars()` | `.collect::<String>()` （[[drill-a39]] [[drill-a46]]） |

`Vec<char>` に `.join()` は **使えない**（join は文字列スライス用）。これを取り違えるのが定番。

## スライス: バイト境界に注意

```rust
&s[l-1..r]      // バイトインデックス。ASCII限定なら文字数と一致（最速、[[drill-a46]]）
v[l-1..r].iter().collect::<String>()   // Vec<char>経由なら Unicode 安全
```

日本語が混じると `&s[..]` は文字の途中で切って **panic**。問題の制約を見て選ぶ。

## chars() は値を返す（`.copied()` 不要）

```rust
let rev: String = s.chars().rev().collect();        // chars はそのまま
let rev: String = v.iter().rev().copied().collect();// iter は copied が要る
```

理由は [[references-deref]]（chars は char を生成、iter は参照を借りる）。

## よく出る変換テンプレ

```rust
s.chars().map(|c| /*変換*/).collect::<String>()     // 1文字ずつ変換（[[drill-a42]]）
s.chars().filter(|&c| c == target).count()          // 特定文字を数える（[[drill-a54]]）
let rev: String = s.chars().rev().collect();         // 反転・回文判定（[[drill-a41]]）
n.to_string().chars().map(|c| c.to_digit(10).unwrap())  // 各桁（[[drill-a23]]）
```

## 関連

- [[references-deref]] — `chars()` vs `iter()`、`.copied()` の使い分け
- [[option-result-handling]] — `to_digit` の `Option`、`filter_map`
- [[iterator-catalog]] — `map`/`filter`/`collect` 全般
