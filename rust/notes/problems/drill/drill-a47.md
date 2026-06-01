---
tags:
  - rust基礎
  - HashSet
  - sort
  - dedup
  - 種類数
problem: drill-a47
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a47|問題文]]

N 個の整数の中に何種類の異なる値があるかを出力する。

## 実装アプローチ

**sort + dedup 版（最短）**:
```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    a.dedup();
    println!("{}", a.len());
}
```

**HashSet 版**:
```rust
use std::collections::HashSet;
let set: HashSet<i64> = a.iter().copied().collect();
println!("{}", set.len());
```

---

## 学んだこと

### `sort` + `dedup` で種類数を数える

```rust
a.sort();
a.dedup();
let kinds = a.len();
```

- `dedup` は **隣接した重複** だけ削除する
- 必ず先に `sort` してから `dedup` でないと意味がない
- 結果の `len()` が種類数

知識レベル: 🔵 説明可能

---

### `dedup` の落とし穴

```rust
let mut v = vec![1, 2, 1, 2, 1];
v.dedup();        // [1, 2, 1, 2, 1] そのまま（連続してない）
v.sort();
v.dedup();        // [1, 2]
```

連続した重複しか消さない。順序を保ったまま重複削除したいなら `HashSet` 経由か `IndexSet` 等が必要。

知識レベル: 🟢 実装可能

---

### HashSet 版との比較

| 方法 | 計算量 | 副作用 |
|---|---|---|
| `sort + dedup` | O(N log N) | a が書き換わる、順序失う |
| `HashSet` | O(N) | 追加メモリ、順序失う |

A47 レベルなら sort+dedup で十分。**「あとで `contains` 問い合わせをしたい」「集合演算したい」なら HashSet**。

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a38]] — Vec メソッド早見表（`dedup` を含む）
- [[drill-a48]] — HashMap で出現回数を数える + 番兵テク
- [[hashmap-hashset]] — HashMap/HashSet のまとめ
