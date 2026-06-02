---
tags:
  - 計算量
  - sort
  - dedup
  - hashset
  - select_nth_unstable
  - 降順ソート
  - Reverse
  - rust基礎
problem: drill-b3
date: 2026-06-02
difficulty: drill
---

## 問題

[[../../../contests/drill/b3|問題文]]

$N$ 個の数の中で $K$ 番目に大きい値を求める。

## 実装アプローチ

**提出コード**:
```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [i64; n],
    }
    p.sort();
    let ans = p[p.len() - k];
    println!("{}", ans);
}
```

方針：昇順 `sort()` して後ろから `K` 番目（`len - k`）を取る。**O(N log N)**。制約が緩ければこれで十分。

> このセッションのテーマは「解けたコードを、計算量の観点でどこまで詰められるか／どこは詰めても無駄か」の見極め。

---

## 学んだこと

### K番目に大きい値を求める計算量の階段

| 方法 | 計算量 | いつ使う |
|---|---|---|
| `iter().max()` を K 回（毎回消す） | O(N·K) | K がごく小さい時だけ。汎用解には不向き |
| `sort()`（提出版） | O(N log N) | シンプルで十分速い。実戦の第一候補 |
| `BinaryHeap`（サイズ K の最小ヒープ） | O(N log K) | K ≪ N、ストリーム処理 |
| `select_nth_unstable` | **O(N) 平均** | 「1個だけ欲しい」の理論的ベスト |

- `max` は「K=1 の特殊ケース」に効くツール。K 回繰り返すと O(N·K) で sort より悪化しうる。
- 「K 番目を1個知るだけ」なら全ソートは過剰。下限は「最低 N-1 回は見る」＝ O(N)。

知識レベル: 🟡 雰囲気理解

---

### `select_nth_unstable`（quickselect）が O(N) になる理由

```rust
let idx = p.len() - k;
p.select_nth_unstable(idx);   // idx の位置だけ確定させる
let ans = p[idx];
```

`select_nth_unstable(idx)` の後の配列の状態:
```
[ idx より小さい値が雑に詰まってる | idx番目=確定 | idx より大きい値が雑に詰まってる ]
        ↑ 左の中身は未ソート           ↑ ここだけ正しい    ↑ 右も未ソート
```

- `sort()` は**全ペアの大小関係**を確定 → 情報量が多く O(N log N) 不可避。
- quickselect は「答えがある側」だけ再帰的に絞る → N + N/2 + N/4 + … ≈ **2N = O(N)**。
- 要らない仕事（左右ブロックの中身を並べること）を**サボってる**のが速さの本質。

`sort` vs `select_nth_unstable` の違い＝**「全体を並べる」か「1箇所だけ正位置に置く」か**。

知識レベル: 🟡 雰囲気理解

---

### 降順ソートの3つの書き方

```rust
// ① キーを反転（おすすめ・汎用性が高い）
use std::cmp::Reverse;
p.sort_by_key(|&x| Reverse(x));

// ② 比較関数を逆に（b が先に来ると降順）
p.sort_by(|a, b| b.cmp(a));

// ③ 昇順 → 逆順（今まで使ってたやつ）
p.sort();
p.reverse();
```

降順にすれば「K 番目に大きい = `p[k-1]`」と 0-indexed で素直に書けて、`len - k` の添字計算より事故りにくい。

**覚え方**:
- `b.cmp(a)` ←「**b が先**で降順」。`a.cmp(b)` が昇順、と対で覚える。
- `Reverse(x)` ←「キーを裏返せば昇順ソートが降順になる」。`BinaryHeap` の最小ヒープと**同じ道具**。

知識レベル: 🟢 実装可能

---

### 降順一行 vs `sort()`+`reverse()` は計算量が同じ

| 方法 | 計算量 |
|---|---|
| `sort_by_key(\|&x\| Reverse(x))` | O(N log N) |
| `sort()` + `reverse()` | O(N log N) + O(N) = **O(N log N)** |

- `reverse()` は配列を1パス舐めるだけの **O(N)**。
- Big-O は最大項だけ残すので O(N) は O(N log N) に飲み込まれて消える。
- → **オーダーは横並び。速さのために降順一行を覚える必要はない。**

`Reverse` を覚える価値があるのは計算量ではなく:
1. **複合ソート**（点数は降順・同点ならIDは昇順）では `reverse()` だと両方ひっくり返って壊れる。`sort_by_key` でないと書けない。
2. `BinaryHeap` の最小ヒープでも同じ `Reverse` が出てくる（道具の使い回し）。

→ **「複合キーが出てきた時が `Reverse` の覚えどき」**。単一キー降順は `sort`+`reverse` のままでいい。

知識レベル: 🔵 説明可能

---

### `sort`+`dedup` vs `HashSet`（重複除去）はオーダーが変わる

ここは reverse の話と違って**計算量の階級が下がる**:

| 方法 | 計算量 |
|---|---|
| `sort()` + `dedup()` | O(N log N)（ソートが支配的） |
| `HashSet` に collect | **O(N) 平均** |

- `dedup()` は**隣り合う重複**しか消さないので必ず先に `sort()` が要る（だから O(N log N) に引き上げられる）。
- `HashSet` は insert が平均 O(1) → 全体 O(N)。

**ただし実戦では定数倍に注意**:
- `HashSet` はハッシュ計算・キャッシュミスで定数倍が大きい。
- `sort`+`dedup` は連続メモリを舐めるのでキャッシュに優しい。
- N が数万〜十数万なら `sort`+`dedup` の方が**実測で速い**ことも普通。

**使い分けの軸＝「ソート済みの結果も欲しいか」**:
- 重複除去して**並べたい** → `sort`+`dedup`（どうせソートするので一石二鳥）。
- **種類数 / 存在判定だけ**（順序不要）→ `HashSet`（`.len()` で種類数）。

```rust
use std::collections::HashSet;
let set: HashSet<_> = p.into_iter().collect();
let kinds = set.len();   // 種類数
```

知識レベル: 🟢 実装可能

---

## このセッションの総括（計算量を詰める価値の見極め）

| 置き換え | オーダー変化 | 覚える価値 |
|---|---|---|
| `sort` → `select_nth_unstable` | O(N log N) → **O(N)** | あり（最速を知っておく） |
| `sort`+`reverse` → 降順一行 | 変化なし（両方 O(N log N)） | 計算量目的では不要 |
| `sort`+`dedup` → `HashSet` | O(N log N) → **O(N)**（ただし定数倍で逆転も） | 順序不要なら価値あり |

**学び**: 「一行で書ける別解」が必ずしも速いわけではない。オーダーが下がるのか（select_nth, HashSet）、定数倍のおまけだけか（reverse）を見分けるのが大事。

---

## 関連ノート

- [[drill-b2]] — `sort` + `dedup` で重複除去・人気投票（同じ dedup の話）
- [[drill-b1]] — `iter().filter` と参照の枚数
