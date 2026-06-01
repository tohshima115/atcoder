---
tags:
  - topic
  - 型推論
  - ターボフィッシュ
  - sum
  - collect
type: topic
date: 2026-06-01
---

# 型推論が要るメソッド と ターボフィッシュ

> **繰り返し課題**: `sum()` / `collect()` を書くたびに「型が決まらない」エラーで詰まる。
> 出てきた問題: [[drill-a23]] [[drill-a35]] [[drill-a49]] [[drill-a53]]

## 何が起きているか

`sum()` と `collect()` は **「集めた先の型」が文脈から決まらないとコンパイルできない**。
左辺の型注釈もターボフィッシュも無いと「型が曖昧」と言われる。

```rust
let s = a.iter().sum();        // ❌ どの数値型に集める？ → エラー
let v = (1..=n).collect();     // ❌ Vec? HashSet? → エラー
```

## 解決の2択（どちらでも同じ）

```rust
// ① 左辺に型注釈
let s: i64 = a.iter().sum();
let v: Vec<usize> = (1..=n).collect();

// ② 右辺にターボフィッシュ ::<T>
let s = a.iter().sum::<i64>();
let v = (1..=n).collect::<Vec<usize>>();
```

- チェーンの途中で型を確定させたいとき・右辺だけ読んで型を知りたいときは **ターボフィッシュ**が便利（[[drill-a49]]）
- 文字列に集めるのも同じ: `chars().collect::<String>()`（[[drill-a39]] [[drill-a46]]）

## 対になる事実: `max()` / `min()` は型推論不要

```rust
let mx = *a.iter().max().unwrap();   // 型注釈不要（要素型で決まる）
```

`max()/min()` は要素の型そのままなので曖昧にならない。代わりに `Option<&T>` を返すので `unwrap()` + `*` が要る → [[option-result-handling]] / [[references-deref]]。

## 早見表

| メソッド | 型指定が要る？ | 理由 |
|---|---|---|
| `sum()` | 要る | 集計先の数値型が未確定 |
| `product()` | 要る | 同上 |
| `collect()` | 要る | 集める容器（Vec/String/HashSet…）が未確定 |
| `max()` / `min()` | 不要 | 要素型のまま |
| `count()` | 不要 | 常に `usize` |

## 関連

- [[references-deref]] — `*` や `.copied()` で参照を外す話
- [[option-result-handling]] — `max().unwrap()` の Option 処理
- [[iterator-catalog]] — collect で作れる容器の一覧
