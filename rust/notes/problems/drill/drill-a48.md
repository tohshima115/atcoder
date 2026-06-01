---
tags:
  - rust基礎
  - HashMap
  - 番兵
  - sort
  - 出現回数
  - イディオム
problem: drill-a48
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a48|問題文]]

N 個の整数のうち、ちょうど1回だけ出現する値が何個あるかを出力する。

## 実装アプローチ

**ソート + 番兵 + 隣接比較版（実装したやつ）**:
```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort();
    a.insert(0, -1);     // 左番兵（A_i は 0..=1000 なので -1 は絶対重複しない）
    a.push(1001);        // 右番兵
    let mut ans: i64 = 0;
    for i in 1..=n {
        if a[i-1] != a[i] && a[i] != a[i+1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
```

**HashMap で出現回数を数える版（B問題以降で頻出）**:
```rust
use std::collections::HashMap;
let mut count: HashMap<i64, i64> = HashMap::new();
for &x in &a {
    *count.entry(x).or_insert(0) += 1;
}
let ans = count.values().filter(|&&v| v == 1).count();
println!("{}", ans);
```

---

## 学んだこと

### 番兵テク — 境界の特殊ケースを消す古典手法

```rust
a.sort();
a.insert(0, -1);     // 左端の番兵
a.push(1001);        // 右端の番兵
```

- 「両端の要素はどう判定する？」で特別扱いが必要になるところに、**到達しない値**を置くことで境界判定を消す
- 制約 `0 ≤ A_i ≤ 1000` なので `-1` と `1001` は元の数列と絶対衝突しない
- これにより `a[i-1] != a[i] && a[i] != a[i+1]` の一本判定が全要素に適用可能

**番兵に使う値の選び方**: 「制約の範囲外」かつ「処理上邪魔にならない」値。最小値より小さい / 最大値より大きいやつ。

知識レベル: 🔵 説明可能

---

### ソート後の「両隣と違う」＝「1回しか出現しない」

ソートすると同じ値は必ず隣接する。なので:

| 条件 | 意味 |
|---|---|
| `a[i-1] != a[i] && a[i] != a[i+1]` | 両隣と違う → ユニーク |
| `a[i-1] == a[i] || a[i] == a[i+1]` | どちらかと同じ → 2回以上出現 |

これを全要素でチェックすれば、ちょうど1回出現する値の個数が数えられる。

知識レベル: 🟢 実装可能

---

### `HashMap` 出現回数イディオム — `entry().or_insert()` 

```rust
use std::collections::HashMap;
let mut count: HashMap<i64, i64> = HashMap::new();
for &x in &a {
    *count.entry(x).or_insert(0) += 1;
}
```

- `entry(x)` — キー `x` の場所を表す `Entry` を返す
- `.or_insert(0)` — そのキーが無ければ 0 を挿入。挿入後の値への可変参照 `&mut V` を返す
- `*... += 1` — その参照経由で値を1増やす

**Python の `defaultdict(int)` と同じ感覚**。「キーがあれば+1、無ければ1から開始」が1行で書ける。

知らないと:
```rust
if let Some(v) = count.get_mut(&x) {
    *v += 1;
} else {
    count.insert(x, 1);
}
```
と長くなる。

知識レベル: 🔵 説明可能

---

### 出現回数からの集計

```rust
let ans = count.values().filter(|&&v| v == 1).count();
```

- `count.values()` — `&V` のイテレータ（値だけ）
- `filter(|&&v| v == 1)` — 値が1のものだけ通す（`&&` で2重参照外し、[[drill-a30]] と同じ）
- `count()` — 個数

`filter + count` パターンの応用。

知識レベル: 🟢 実装可能

---

## 3つのアプローチ比較（A47, A48 を通じて）

| やりたいこと | アプローチ | 計算量 |
|---|---|---|
| 種類数 | `sort + dedup + len` | O(N log N) |
| 種類数（順序気にしない、追加メモリOK） | `HashSet` | O(N) |
| 出現回数を集計 | `HashMap` + `entry().or_insert(0) += 1` | O(N) |
| 1回出現の検出 | ソート + 番兵 + 隣接比較 | O(N log N) |
| 1回出現の検出（HashMap） | `HashMap` + `filter(==1).count()` | O(N) |

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a47]] — `HashSet` で種類数
- [[ref-hashmap-hashset]] — HashMap / HashSet のまとめ
- [[drill-a30]] — `filter` の `|&&x|` パターン
- [[drill-a38]] — `insert` / `push` の Vec メソッド
