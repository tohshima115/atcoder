---
tags:
  - BTreeMap
  - entry
  - or_default
  - 集計
  - rust基礎
problem: drill-b17
date: 2026-06-04
difficulty: drill
---

## 問題

[[../../../contests/drill/b17|問題文]]

$N$ 個の整数について「値 と 出現回数」を**値の昇順**で出力する。

## 実装アプローチ

`BTreeMap` で集計 → そのまま回すだけ（キーが自動で昇順なので sort 不要）。

```rust
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! { n: usize, a: [i64; n] }
    let mut count: BTreeMap<i64, i64> = BTreeMap::new();
    for &key in &a {
        *count.entry(key).or_default() += 1;
    }
    for (&key, &value) in &count {     // BTreeMap は key 昇順で出る → sort 不要
        println!("{} {}", key, value);
    }
}
```

[[drill-b14]]（HashMap）では最後に `sort()` が要ったが、BTreeMap なら**集計してそのまま回すだけ**。「集計＋昇順出力」は BTreeMap が一番ラク、を体感。

---

## 学んだこと

### `*map.entry(key).or_default() += 1` を3つの問いで組み立てる

まだチートシートを見ないと書けない段階だったので、**暗記でなく毎回組み立てる**方法:

| 問い | 答え | 書くもの |
|---|---|---|
| ① どのキーをいじる？ | `key` の席へ | `map.entry(key)` |
| ② 無かったらどうする？ | 既定値(0)を置いて、値への手をもらう | `.or_default()` |
| ③ その値をどうする？ | 中身を +1 | `*… += 1` |

「**どのキー → 無ければどうする → どう更新**」の順で唱えながら書く。

知識レベル: 🟢 実装可能

### `*` を忘れない：`or_default()` は「値への手（&mut）」を返す

`or_default()` の返りは `&mut i64`（値そのものでなく手）。手のままでは足せないので `*` で開ける。
迷ったら**2 行に開く**と関係が見える:

```rust
let counter = list.entry(key).or_default();  // counter: &mut i64（手）
*counter += 1;                                // 手を開いて中身を +1
```

`+= 1`（演算子）は `*` が要るが、`.push(v)`（メソッド）は auto-deref が効くので `*` 不要、という違いも整理:

```rust
*count.entry(k).or_default() += 1;       // 演算子 → * 要る
groups.entry(k).or_default().push(v);    // メソッド → * 不要
```

知識レベル: 🟡 雰囲気理解（あと数回書けば 🟢）

---

## 関連ノート

- [[../../reference/hashmap-hashset]] — entry API・`or_default()`
- [[drill-b14]] — HashMap 版（sort が要る／BTreeMap との対比）
- [[../../topics/references-deref]] — `&mut` と `*` の関係
