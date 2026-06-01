---
tags:
  - topic
  - 参照
  - 参照外し
  - copied
  - クロージャ
type: topic
date: 2026-06-01
---

# 参照と参照外し（&・*・copied・クロージャの &&）

> **繰り返し課題**: `filter` の `|&&x|`、`map` の `|&x|`、`max().unwrap()` の後の `*`、`.copied()`…参照がいつ何枚付くか分からず「呪文」で乗り切っている。
> 出てきた問題: [[drill-a21]] [[drill-a30]] [[drill-a35]] [[drill-a41]] [[drill-a43]] [[drill-a54]]

> 競プロで扱うのはほぼ `Copy` 型（`i64`,`char`,`usize`…）。**最初は呪文で丸暗記でOK**、ここはRustで一番難しい所。

## 3種のイテレータ — 参照 vs 値

| メソッド | 要素の型 | 元 |
|---|---|---|
| `vec.iter()` | `&T`（参照） | 借りる |
| `vec.into_iter()` | `T`（値） | 消費する |
| `s.chars()` | `char`（値） | 生成系（借りるが値で返す） |

`chars()` は UTF-8 をデコードして `char` を**生成**するので最初から値 → `.copied()` 不要で `collect::<String>()` できる（[[drill-a41]]）。`iter()` は借りるだけなので `&T`、値が要る場面で外す必要がある。

## クロージャの `&` 枚数

| 書き方 | 場面 | なぜ |
|---|---|---|
| `\|&&x\|` | `filter` / `find` | `iter()` の `&T` に filter 側がもう1枚 `&` を足す（[[drill-a30]] [[drill-a52]]） |
| `\|&x\|` | `map` で値として使う | 参照1枚を外して `x: T` |
| `\|x\|` | `map` で `.to_string()` 等を呼ぶ | auto-deref で参照のまま呼べる（[[drill-a35]]） |

```rust
.filter(|&&x| x >= k)   // = .filter(|x| **x >= k)
.map(|&x| x * 2)
.map(|x| x.to_string()) // 参照のまま OK
```

## `max().unwrap()` の後の `*`

```rust
*a.iter().max().unwrap()
// max() → Option<&i64> / unwrap() → &i64 / * → i64
```

`iter()` 由来なので参照、計算に使うには `*` で外す（[[drill-a21]]）。値が欲しいだけなら [[option-result-handling]] も参照。

## `.copied()` — `|&&x|` 地獄からの脱出

```rust
let rev: String = v.iter().rev().copied().collect();   // &char → char
let set: HashSet<i64> = a.iter().copied().collect();
// これらは全部同じ意味
.copied()   .map(|x| *x)   .map(|&x| x)
```

| 道具 | いつ |
|---|---|
| `.copied()` | 中身が `Copy`（`i64`,`char`,`bool`,`usize`…）← 競プロはほぼこれ |
| `.cloned()` | `Clone` だが `Copy` でない（`String`,`Vec`…） |

⚠️ `*` は **1個**の参照外し演算子。`*v.iter()...collect()` のようにイテレータ全体には効かない。要素ごとに `.map(|x| *x)` か `.copied()`（[[drill-a41]]）。

## `for` ループでの参照パターン（[[drill-a43]]）

```rust
for (x, v) in queries  { ... }   // 消費。x: usize, v: i64（もう使わないなら最楽）
for &(x, v) in &queries { ... }  // 借りる＋&を外して値。あとで使うなら
for (&x, &v) in &queries { ... } // 中身ごとに外す（上と等価）
for &v in &a { ... }             // 借りて値で受ける定番
```

`for ... in v` は暗黙に `v.into_iter()`（消費）。あとで使うなら `&v` で借りる。

## 関連

- [[iterator-catalog]] — このクロージャ規則が効くメソッド群
- [[string-char-conversion]] — `chars()` が値を返す話
- [[option-result-handling]] — `Option<&T>` の扱い
