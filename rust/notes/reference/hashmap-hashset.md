---
tags:
  - rust基礎
  - HashMap
  - HashSet
  - 標準ライブラリ
  - リファレンス
problem: ref-hashmap-hashset
date: 2026-06-01
difficulty: reference
---

# HashMap と HashSet まとめ

競プロで使う `std::collections::HashMap` / `HashSet` の頻出操作を一覧化。B問題以降で多用するので脳内に入れておく。

```rust
use std::collections::{HashMap, HashSet};
```

---

## HashSet — 重複なしの集合

### 作成

```rust
let mut s: HashSet<i64> = HashSet::new();

// Vec から作る
let s: HashSet<i64> = a.iter().copied().collect();
let s: HashSet<i64> = a.into_iter().collect();   // 消費版

// リテラル
let s: HashSet<i64> = [1, 2, 3].into_iter().collect();
```

### 主な操作

| やりたいこと | コード | 計算量 |
|---|---|---|
| 追加 | `s.insert(x)` | O(1) |
| 削除 | `s.remove(&x)` | O(1) |
| 存在確認 | `s.contains(&x)` | O(1) |
| 要素数 | `s.len()` | O(1) |
| 反復 | `for x in &s { ... }` | O(N) |
| 空判定 | `s.is_empty()` | O(1) |

### 集合演算

```rust
let a: HashSet<i64> = ...;
let b: HashSet<i64> = ...;

// 和集合
let u: HashSet<i64> = a.union(&b).copied().collect();
// 積集合
let i: HashSet<i64> = a.intersection(&b).copied().collect();
// 差集合 (a - b)
let d: HashSet<i64> = a.difference(&b).copied().collect();
```

### 使いどころ
- 種類数を数える（[[drill-a47]]）
- 「既に出たか」を O(1) で問い合わせ
- 重複削除（順序は失う）

知識レベル: 🟢 実装可能

---

## HashMap — キーと値の写像

### 作成

```rust
let mut m: HashMap<i64, i64> = HashMap::new();

// 初期データから
let m: HashMap<&str, i64> = [("a", 1), ("b", 2)].into_iter().collect();
```

### 主な操作

| やりたいこと | コード | 計算量 |
|---|---|---|
| 挿入・上書き | `m.insert(k, v)` | O(1) |
| 取得（Option） | `m.get(&k)` → `Option<&V>` | O(1) |
| 存在確認 | `m.contains_key(&k)` | O(1) |
| 削除 | `m.remove(&k)` | O(1) |
| 要素数 | `m.len()` | O(1) |
| 反復 | `for (k, v) in &m { ... }` | O(N) |
| キー一覧 | `m.keys()` | O(N) |
| 値一覧 | `m.values()` | O(N) |

### 競プロ頻出イディオム

**① 出現回数を数える**（[[drill-a48]]）:
```rust
let mut count: HashMap<i64, i64> = HashMap::new();
for &x in &a {
    *count.entry(x).or_insert(0) += 1;
}
```

**② 値を蓄積（グループ化）**:
```rust
let mut groups: HashMap<i64, Vec<i64>> = HashMap::new();
for &(k, v) in &pairs {
    groups.entry(k).or_insert_with(Vec::new).push(v);
}
```

**③ 値の取得 + デフォルト**:
```rust
let v = *m.get(&k).unwrap_or(&0);
```

**④ 条件を満たすキーをカウント**:
```rust
let ans = count.values().filter(|&&v| v == 1).count();
```

知識レベル: 🔵 説明可能

---

## `entry` API の整理

`entry(key)` は `Entry` を返す。これに対するメソッド:

| メソッド | 意味 |
|---|---|
| `.or_insert(v)` | キー無ければ `v` を挿入。値への `&mut V` を返す |
| `.or_insert_with(\|\| ...)` | クロージャで初期値を生成（Vec::new などに使う） |
| `.or_default()` | `Default::default()` を初期値に |
| `.and_modify(\|v\| ...)` | キーが既にあるときだけ実行 |

```rust
// or_insert: 数値カウント用
*m.entry(k).or_insert(0) += 1;

// or_insert_with: 重い初期化（Vec, String など）
m.entry(k).or_insert_with(Vec::new).push(v);

// or_default: Default を持つ型なら最短
*m.entry(k).or_default() += 1;
```

知識レベル: 🟡 雰囲気理解

---

## 順序が必要なときは `BTreeMap` / `BTreeSet`

| 要件 | 選ぶ型 |
|---|---|
| 順序関係ない、最速 | `HashMap` / `HashSet` |
| キーをソート順で反復したい | `BTreeMap` / `BTreeSet` |
| 範囲クエリ（key in [a..b]）したい | `BTreeMap` / `BTreeSet` |

`BTreeMap` は同じインターフェイスで使えるが、操作が O(log N) になる。代わりに `.range(l..r)` などが使える。

知識レベル: 🟡 雰囲気理解

---

## トラブルシューティング

### `HashMap::new()` で型が決まらない

```rust
let mut m = HashMap::new();   // 型不明エラー
let mut m: HashMap<i64, i64> = HashMap::new();   // OK
```

ターボフィッシュでもOK: `HashMap::<i64, i64>::new()`

### `get` が `Option<&V>` を返すので `*` が必要

```rust
let v: i64 = *m.get(&k).unwrap();
let v: i64 = *m.get(&k).unwrap_or(&0);
```

### キーの参照を取る `&k`

`get(&k)`, `contains_key(&k)`, `remove(&k)` は全部キーの参照を取る。`k` がそのまま使えると思って `k` を渡すとエラーになりがち。

---

## 関連ノート

- [[drill-a47]] — `HashSet` で種類数
- [[drill-a48]] — `HashMap` で出現回数 + `entry().or_insert()` イディオム
- [[drill-a30]] — `filter` の参照外し（HashMap の values でも使う）
