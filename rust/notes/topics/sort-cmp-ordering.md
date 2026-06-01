---
tags:
  - topic
  - ソート
  - cmp
  - Ordering
  - dedup
type: topic
date: 2026-06-01
---

# ソート・比較・Ordering と破壊的 Vec 操作

> **繰り返し課題**: 降順ソートの呪文、`sort()` が `()` を返してチェーンできない、`dedup` が連続重複しか消さない、で毎回つまずく。
> 出てきた問題: [[drill-a28]] [[drill-a29]] [[drill-a44]] [[drill-a47]]

## ソート（`mut` 必須・その場で書き換え）

```rust
a.sort();                     // 昇順
a.sort_by(|x, y| y.cmp(x));   // 降順（x と y を入れ替える呪文、[[drill-a28]] [[drill-a29]]）
a[l-1..r].sort();             // 区間だけソート
```

- `sort()` は **`()` を返す** → `.rev()` をチェーンできない。逆順で読むなら `a.iter().rev()`（[[drill-a29]]）
- `f64` は `NaN` のせいで `sort()` できない → [[integer-types-overflow]]

## 降順上位k個 / 中央値

```rust
a.sort();
a[(n-k)..].iter().sum::<i64>()        // 昇順後ろからk個（[[drill-a29]]）
a.iter().rev().take(k).sum::<i64>()   // 同じ意味
a[(n-1)/2]                             // 奇数個の中央値（[[drill-a28]]）。(n+1)/2 はズレる
```

## 比較は `cmp` → `Ordering`

```rust
use std::cmp::Ordering;
match a.cmp(&b) {                  // 引数は参照 &b
    Ordering::Greater => "Greater",
    Ordering::Less    => "Less",
    Ordering::Equal   => "Equal",
}
```

3値出力にハマりすぎな API（[[drill-a44]]）。`match` で網羅すれば `unreachable!()` 不要 → [[match-exhaustiveness]]。

## 種類数・重複削除

```rust
a.sort(); a.dedup(); a.len()       // 種類数。dedup は必ず sort の後（[[drill-a47]]）
let set: HashSet<_> = a.iter().copied().collect();  // O(N) 版
```

⚠️ `dedup` は **隣接した重複しか消さない**。`[1,2,1,2]` は `sort` してからでないと残る（[[drill-a47]]）。

## 破壊的 Vec 操作 早見表

| 操作 | メソッド | 戻り値 |
|---|---|---|
| 反転（全体/区間） | `a.reverse()` / `a[l..r].reverse()` | `()`（[[drill-a36]]） |
| 入れ替え | `a.swap(i, j)` | `()`（[[drill-a37]]） |
| 挿入 / 末尾追加 | `a.insert(pos, x)` / `a.push(x)` | `()` / `()`（[[drill-a38]]） |
| 末尾取り出し / 削除 | `a.pop()` / `a.remove(pos)` | `Option<T>` / 要素 |
| ローテート | `a.rotate_left(k)` / `rotate_right(k)` | `()`（[[drill-a51]]） |
| 重複削除（隣接） | `a.dedup()` | `()` |

破壊系は戻り値が `()` のものが多い → チェーンせず文として書く。読み取り専用で並べ替えたいだけなら `.iter().rev()` など非破壊版を使う。

## 関連

- [[match-exhaustiveness]] — `Ordering` を match で網羅
- [[iterator-catalog]] — 非破壊の `rev`/`take`/`chain`
- [[integer-types-overflow]] — f64 がソートできない理由
