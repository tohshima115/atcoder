---
tags:
  - topic
  - イテレータ
  - メソッドチェーン
  - filter
  - map
type: topic
date: 2026-06-01
---

# イテレータ操作カタログ

> **繰り返し課題**: やりたい操作に対応するメソッドが思い出せず、ループで自力実装してハマる。「知ってるか知らないか」ゲー。
> 出てきた問題: [[drill-a20]] [[drill-a29]] [[drill-a30]] [[drill-a35]] [[drill-a38]] [[drill-a50]] [[drill-a51]] [[drill-a52]]

## まず `.iter()` でイテレータに変換

`Vec<T>` の `.sum()`/`.map()`/`.filter()` は **イテレータ前提**なので先に `.iter()` が要る。
一方 `.abs()`/`.max(b)`/`.min(b)` は **値そのもの**のメソッドなので `.iter()` 不要（[[drill-a20]]）。

`.iter()` / `.into_iter()` / `.chars()` の違い → [[references-deref]]。

## 各要素を処理する

```rust
.map(|x| x * 2)              // 変換
.filter(|&&x| x >= k)        // 条件で絞る（filter は &&、[[references-deref]]）
.filter_map(|c| c.to_digit(10))  // 変換+None除外（[[option-result-handling]]）
.enumerate()                 // (index, value) のペアに
```

## 集計する（終端操作）

```rust
.sum::<i64>()    .product::<i64>()    // 型指定が要る（[[sum-collect-turbofish]]）
.max()  .min()                        // Option<&T> を返す（[[option-result-handling]]）
.count()                              // usize
.collect::<Vec<_>>()  .collect::<String>()  // 容器に集める
.all(|w| w[0] < w[1])   .any(...)     // 全部/どれか
```

「条件を満たす個数」は **`filter(...).count()`** 一択（[[drill-a30]] [[drill-a54]]）。

## 探す

| やりたいこと | メソッド | 戻り値 |
|---|---|---|
| **位置**が欲しい | `position(\|&v\| ...)` | `Option<usize>` |
| **値**が欲しい | `find(\|&&v\| ...)` | `Option<&T>` |
| 含むか | `contains(&x)` | `bool` |

`find`/`position` は **クロージャを取る**（値を直接渡せない）。`find` は `&&`、`position` は `&`（[[drill-a52]]）。

## 取る・連結・並べる

```rust
.take(k)   .skip(k)          // 先頭k個 / 先頭k個を飛ばす（[[drill-a29]]）
.rev()                       // 逆順イテレータ（元配列は変えない）
a.iter().chain(b.iter())     // 2つを連結（[[drill-a51]]）
.windows(2)                  // 連続k要素のスライス。境界処理が消える（[[drill-a50]]）
.chunks(2)                   // 重ならずk個ずつ
```

`windows(k)` は隣接ペア系の決定版: `a.windows(2).map(|w| w[1]-w[0]).max()`（[[drill-a50]]）。

## レンジから作る

```rust
(1..=n).collect::<Vec<usize>>()   // ..= は n を含む、.. は含まない（[[drill-a35]]）
(1..=n).map(|x| x as i64).sum()
```

## ループ自力実装 vs メソッド

A問題の数列・文字列操作は大体メソッド1行になる。詰まったら **やりたい動作の英単語**（swap, insert, reverse, rotate, position…）で std doc を検索（[[drill-a38]] [[drill-a51]]）。Vec破壊系メソッドは [[sort-cmp-ordering]] とこのカタログに集約。

## 関連

- [[references-deref]] — `|&&x|` `|&x|` `|x|` の使い分け（イテレータの肝）
- [[sum-collect-turbofish]] — `sum`/`collect` の型指定
- [[option-result-handling]] — `max`/`position` の Option 処理
- [[sort-cmp-ordering]] — `sort`/`dedup`/`reverse` などの破壊的操作
