---
tags:
  - BTreeMap
  - range
  - 範囲クエリ
  - Option
  - unwrap_or
  - rust基礎
problem: drill-b16
date: 2026-06-04
difficulty: drill
---

## 問題

[[../../../contests/drill/b16|問題文]]

$N$ 件のイベント `(時刻 t, 値 v)`。各質問 $x$ について「**時刻 $x$ 以上で最も早いイベント**の値」を答える。無ければ `-1`。

## 実装アプローチ

`BTreeMap<時刻, 値>` を作り、`range(x..).next()` で「$x$ 以上で最小のキー」のエントリを取る。

```rust
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! { n: usize, q: usize, event: [(i64, i64); n], x: [i64; q] }
    let m: BTreeMap<i64, i64> = event.iter().copied().collect();
    for i in 0..q {
        let ans = match m.range(x[i]..).next() {
            Some((_k, &v)) => v,   // x 以上で最小キーの値
            None => -1,            // 該当なし
        };
        println!("{}", ans);
    }
}
```

---

## 学んだこと

### `range(x..).next()` の分解

| 式 | やってること | 返り値 |
|---|---|---|
| `m.range(x..)` | キーが `[x, ∞)` のエントリを**昇順**で並べるイテレータ | `Range`（イテレータ） |
| `.next()` | 先頭1個＝**x 以上で最小キー**のエントリを取る | `Option<(&i64, &i64)>` |

BTreeMap がキー昇順だから「range で範囲を切る → next で先頭」で「x 以上で最小」が取れる。
該当なしのとき `next()` は `None` を返す（→ これが `-1` のケース）。

知識レベル: 🟢 実装可能

### `range` は必ず「キー」にかかる（[[drill-b18]] で混乱した点）

`range`・`first_key_value`・`last_key_value` は**すべてキー基準**。値で範囲検索はできない。
b16 で上手くいったのは、**探したい「時刻」がキーだった**から。
→ 「値で探したい」問題（[[drill-b18]]）には BTreeMap range は使えない。**探したいものをキーにする**のが鉄則。

知識レベル: 🔵 説明可能

### クエリ結果の Option は `or_insert` でなく `map`/`unwrap_or`

「無ければ -1」を `or_insert(-1)` と書きたくなるが**それは誤り**。`or_insert` は `entry()` の続き（map に書き込む用）。
`range().next()` の結果は**クエリ結果としての `Option`** なので、Option 用の道具を使う:

```rust
// match 版
let ans = match m.range(x..).next() { Some((_, &v)) => v, None => -1 };
// チェーン版
let ans = m.range(x..).next().map(|(_, &v)| v).unwrap_or(-1);
```

| 状況 | 道具 |
|---|---|
| map に「無ければ入れて更新」 | `entry(k).or_default()` / `or_insert(v)` |
| クエリ結果の Option から「無ければ既定値」 | `.map(...).unwrap_or(既定)` |

`or_insert` は map をいじる、`unwrap_or` は Option から取り出す。名前が似てるので混同注意。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[../../reference/map-when-and-btreemap]] — BTreeMap の range/first/last イディオム
- [[drill-b18]] — range が「キー専用」で使えない例（対比）
- [[../../topics/option-result-handling]] — Option の map/unwrap_or
