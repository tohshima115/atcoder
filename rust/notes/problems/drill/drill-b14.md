---
tags:
  - HashMap
  - BTreeMap
  - entry
  - or_default
  - 参照
  - join
  - 集計
  - rust基礎
problem: drill-b14
date: 2026-06-04
difficulty: drill
---

## 問題

[[../../../contests/drill/b14|問題文]]

$N$ 個の整数のうち、最も多く出現する値を**すべて**、**小さい順**に出力する。

## 実装アプローチ

**提出コード**（自力 AC）:
```rust
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! { n: usize, a: [i64; n] }
    let mut count: HashMap<i64, i64> = HashMap::new();
    for &k in &a {
        *count.entry(k).or_default() += 1;     // 出現回数を集計
    }
    let max = *count.values().max().unwrap();  // 最大の回数
    let mut list: Vec<i64> = Vec::new();
    for (&k, &v) in &count {                    // 回数が max のキーを集める
        if v == max { list.push(k); }
    }
    list.sort();                                // 昇順
    let ans: Vec<String> = list.iter().map(|&x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
```

構造：**集計 → max 取得 → max のキーを絞り込み → sort → join 出力**。この型は「最頻値を全部」系の定番。

**改善案（BTreeMap で `sort()` を消す）**: 下の学び参照。

> [[drill-b12]] は「最頻値 1 つ」で sort+dedup で逃げられたが、**複数答え**になると HashMap 集計が要る = Map の逃げ道の限界（[[../../reference/map-when-and-btreemap]]）。

---

## 学んだこと

### `or_default()` 一本でカウントできる

```rust
*count.entry(k).or_default() += 1;
```

`or_insert(0)` と同じ（`i64` の規定値が `0`）だが初期値を書かなくていい。**普段のカウント／グループ化は `or_default()` 一本でOK**。初期値が 0/空 以外のときだけ `or_insert(値)`（例：最小値追跡の `or_insert(i64::MAX)`）。詳細 [[../../reference/hashmap-hashset]]。

知識レベル: 🟢 実装可能

### HashMap の絞り込みは `keys()` でなく `&map` を回す

`count.keys()` は**キーだけ**でし、値（回数）が見えないので「回数が max か」を判定できない。
キーと値の両方が要るので **`&count` をそのまま回して `(キー, 値)`** で見る。

```rust
for (&k, &v) in &count { if v == max { list.push(k); } }
```

知識レベル: 🟢 実装可能

### HashMap のイテレートは `(&K, &V)`（各フィールドに参照）

ここが [[../../topics/references-deref]] の核心の再確認。`&count` を回すと各要素は **`(&i64, &i64)`**。
Vec の `&(i64,i64)`（参照は**外側に1枚**）と違い、HashMap は **`(&キー, &値)`（各フィールドに参照）**。

| 回す対象 | 要素の型 | 剥がし方 |
|---|---|---|
| `&Vec<(K,V)>`（[[drill-b8]]） | `&(K, V)`（外側1枚） | `for &(k, v)` |
| `&HashMap<K,V>`（b14） | `(&K, &V)`（各フィールド） | `for (&k, &v)` |

→ b8 では `(&op,&x)` がエラーだったのに、b14 では `(&k,&v)` が**通る**。理由は「参照がどこに付くか」が違うから。同じ `(&_, &_)` でも対象次第で OK/NG が逆になる。

知識レベル: 🔵 説明可能

### スペース区切り出力は `join` の前に文字列化

`Vec<i64>` は `println!("{}", v)` で直接出せない。各数を文字列にしてから `join`:

```rust
let ans: Vec<String> = list.iter().map(|&x| x.to_string()).collect();
println!("{}", ans.join(" "));   // "2 3"
```

`join(" ")` は**文字列の Vec にしか効かない**ので、先に `.to_string()` で `Vec<String>` 化が必要。

知識レベル: 🟢 実装可能

### 「昇順出力」なら BTreeMap で `sort()` が消える（改善案）

出力が**キー昇順**なら、`HashMap` + `list.sort()` の代わりに **`BTreeMap`**（キーが常に昇順）を使うと、`for (&k,&v) in &count` で push した時点で `list` はソート済み → **`sort()` 不要**。

```rust
use std::collections::BTreeMap;
let mut count: BTreeMap<i64, i64> = BTreeMap::new();  // 型名を変えるだけ
// ... 集計・絞り込みは同じ ...
// list.sort();  ← 消せる（BTreeMap は k 昇順で出るため）
```

- 「HashMap で書けたら型名を BTreeMap に変えるだけ」が実際に効く例。
- トレードオフ：BTreeMap は各操作 O(log n)、HashMap は O(1)。この規模なら誤差で、**コードのシンプルさ**を取るなら BTreeMap。「集計してキー昇順で出す」系の定番。

知識レベル: 🟡 雰囲気理解

---

## このセッションの総括

| 学び | 要点 |
|---|---|
| カウント | `*map.entry(k).or_default() += 1` 一本 |
| 絞り込み | `keys()` でなく `&map` を `(&k,&v)` で回す |
| 参照の位置 | HashMap は `(&K,&V)`／Vec は `&(K,V)` で剥がし方が逆 |
| 出力 | `join(" ")` の前に `to_string()` で `Vec<String>` 化 |
| 昇順出力 | BTreeMap にすると sort 不要 |

**学び**: 「集計→max→絞り込み→出力」は最頻値系の定番フロー。`or_default()` と「HashMap は `(&k,&v)`」を押さえれば自力で書ける。昇順要件は BTreeMap で素直になる。

---

## 関連ノート

- [[../../reference/hashmap-hashset]] — entry API・`or_default()`・絞り込みイディオム
- [[../../reference/map-when-and-btreemap]] — HashMap/BTreeMap の使い分け（逃げ道の限界）
- [[../../topics/references-deref]] — `(&k,&v)` が通る理由（参照がどこに付くか）
- [[drill-b12]] — 最頻値1つ（sort+dedup で逃げた版）
- [[drill-b8]] — `&(op,x)` の参照（Vec は外側1枚で b14 と対比）
