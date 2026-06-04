---
tags:
  - HashMap
  - BTreeMap
  - 配列で代用
  - 逃げ道
  - 範囲クエリ
  - リファレンス
date: 2026-06-04
difficulty: reference
---

# Map をいつ使うか — 逃げ道の限界と BTreeMap

> 疑問: 「Set は `Vec.sort().dedup()` で代用できる。Map にも逃げ道はある？」
> 答え: **Set の逃げ道は強い（その判断で OK）。Map の逃げ道は限定的で、超えると Map が必須になる。**
> HashMap の基本操作・entry API は [[hashmap-hashset]] を参照。ここは「使い分けの判断」に集中。

---

## Set：逃げ道は強い（まず sort+dedup でよい）

| やりたいこと | 逃げ道 | 逃げ切れる？ |
|---|---|---|
| 重複除去・種類数 | `v.sort(); v.dedup();` → `v.len()` | ✅ ほぼ常に OK |
| 存在判定（1回） | `v.sort(); v.binary_search(&x).is_ok()` | ✅ O(log N) |
| 存在判定（**挿入しながら何度も**） | — | ❌ → `HashSet` |

→ **`HashSet`/`BTreeSet` をわざわざ覚えなくても、当面 sort+dedup で困らない**というあなたの判断は正しい。
Set が必須になるのは「ループの途中で要素を足しながら、その都度 O(1) で存在を問い合わせる」オンライン処理のときだけ。

---

## Map：逃げ道は 2 つ、でも限界がある

### 逃げ道① キーが小さい非負整数 → 配列で代用

```rust
let mut cnt = vec![0i64; MAX + 1];   // 添字 = キー
for &x in &a { cnt[x] += 1; }        // カウントも関連付けも添字で
```

- Map より**速い**（ハッシュ計算なし・キャッシュに優しい）。
- **限界**: キーが大きい（$10^9$）・負・**文字列**・タプルだと配列が作れない。
  - ⚠️ [[drill-b12]] で `a_i ≤ 10^9` だったのは「配列で代用は無理だぞ」というサインだった。

### 逃げ道② バッチ集計なら sort + 連続groupを数える

```rust
a.sort();
// 隣同士を見て、同じ値が続く長さ = 出現回数
```

- [[drill-b12]] で実際にやった手（sort+dedup+count）。**全部出してからまとめて処理**できるなら有効。
- **限界**: 「ストリームの途中でキーを引く」「挿入しながら問い合わせる」**オンライン処理はできない**。

### → Map が必須になる条件

**「キーが大きい/文字列/タプル」かつ「引きながら更新（オンライン）」** の両方が揃ったとき:

- 例: [[drill-b15]] の two-sum を O(N) にする
  （見た値を `HashSet`/`HashMap` に貯め、各要素で「相方 K-x が既出か」を即問い合わせる）
  → sort では「引きながら」ができないので逃げられない。
- 例: 文字列キーの集計（単語の出現回数）→ 配列添字にできない。

| 状況 | 逃げ道 | 結論 |
|---|---|---|
| キー小さい整数・カウント | 配列 `vec![0;N]` | 逃げ切れる |
| バッチで集計（全部見てから） | sort + group | 逃げ切れる |
| キー大/文字列 + オンライン問い合わせ | — | **Map 必須** |

---

## HashMap vs BTreeMap の選択

両者は**ほぼ同じインターフェイス**（`insert`/`get`/`entry`…）。違いは順序と計算量だけ。

| 要件 | HashMap | BTreeMap |
|---|---|---|
| ただ key→value（順序不要） | ◎ O(1) | △ O(log N) |
| キーを**ソート順**で反復 | ✗ 順序バラバラ | ◎ 自動で昇順 |
| **範囲クエリ**（x 以上の最小 等） | ✗ | ◎ `.range()` |
| 最小/最大キー | ✗ | ◎ `first_key_value`/`last_key_value` |

**BTreeMap の逃げ道**: 「集計してキー昇順で出すだけ」なら HashMap に集計 → `keys` を Vec にして sort でも代用可。
でも**オンラインで範囲クエリ**（要素を足しながら「x 以上の最小」を何度も聞く）は sort では無理 → **BTreeMap の独壇場**。

---

## BTreeMap 頻出イディオム

```rust
use std::collections::BTreeMap;
let mut m: BTreeMap<i64, i64> = BTreeMap::new();
m.insert(10, 100);

// ① キー昇順で反復（HashMap と違い自動でソート済み）
for (k, v) in &m { /* k 昇順 */ }

// ② 範囲クエリ: x 以上で最小のキー（無ければ None）
if let Some((&k, &v)) = m.range(x..).next() { /* k は x 以上で最小 */ }

// ③ x 以下で最大のキー（floor）: 逆向きに range して next_back
if let Some((&k, &v)) = m.range(..=x).next_back() { /* k は x 以下で最大 */ }

// ④ 最小・最大キー
if let Some((&k, &v)) = m.first_key_value() { /* 最小 */ }
if let Some((&k, &v)) = m.last_key_value()  { /* 最大 */ }
```

- `range(x..)` = `[x, ∞)`、`range(..=x)` = `(-∞, x]`。`next()` で先頭、`next_back()` で末尾。
- カウントは HashMap と同じく `*m.entry(k).or_insert(0) += 1;`。BTreeMap なら**自動で昇順集計**になる（[[drill-b17]]）。

知識レベル: 🟡 雰囲気理解 →（Stage2 b16/b17 で 🟢 を目指す）

---

## まとめ：判断フロー

1. **キーが小さい非負整数？** → 配列 `vec![0; N]` で代用（最速）
2. **全部見てからの集計でいい（オンライン不要）？** → sort + group で代用
3. 上で逃げられない（キー大/文字列 ＋ 引きながら更新）→ **Map を使う**
   - 順序いらない → **HashMap**
   - キー昇順 / 範囲クエリ / 最小最大 → **BTreeMap**

---

## 関連ノート

- [[hashmap-hashset]] — HashMap/HashSet の基本操作・entry API の詳細
- [[drill-b12]] — sort+dedup で Map を回避した例（逃げ道②）
- [[drill-b15]] — two-sum を Map で O(N)（逃げられない例）
- [[drill-b16]] — BTreeMap の範囲クエリ `.range()`
- [[drill-b17]] — BTreeMap で昇順集計
