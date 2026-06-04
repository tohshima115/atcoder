---
tags:
  - 診断
  - drill
  - HashMap
  - BTreeMap
  - ABC対策
date: 2026-06-04
status: stage2-進行中
---

# Stage 2 — 苦手メカニクス深掘り（HashMap/BTreeMap 中心）

[[diagnostic-stage1]] の結論「弱点は Rust メカニクス（参照/deref・bit全探索・HashMap）」を受けて、
**HashMap/BTreeMap の習熟**を主軸に据えた定着問題（b14〜b18）。配点 300（難 B〜易 C）。

## 進め方

1. b14 〜 b18 を解く（`cct b14` でテスト）。
2. 各問、自己評価（🟢楽勝 / 🟡時間かかった / 🔴解けない）と詰まり所をメモ。
3. 先に **ドキュメント** を読むと効率的:
   - [[../reference/map-when-and-btreemap]] — Map をいつ使うか（逃げ道の限界）と BTreeMap
   - [[../reference/hashmap-hashset]] — HashMap/HashSet の基本操作・entry API

## 問題一覧

| # | 問題 | 狙い | 評価 | メモ |
|---|---|---|---|---|
| b14 | 最頻値をすべて | HashMap 集計→キーを集めて sort | 🟢 自力AC | 集計→max→絞り込み→join。`keys()`では値が見えず`&map`を`(&k,&v)`で回す点、HashMapは各フィールドに参照（Vecと逆）が学び。BTreeMapならsort不要。→[[../problems/drill/drill-b14]] |
| b15 | 和が K のペア(高速) | HashSet で two-sum O(N)（b6 の O(N²)→O(N)） | | |
| b16 | 次のイベント | **BTreeMap** 範囲クエリ `range(x..).next()` | | |
| b17 | 値ごとの個数(昇順) | **BTreeMap** で自動昇順集計 | | |
| b18 | 最高得点者 | 参照/deref 総合（`Vec<(String,i64)>`） | | |

## この 5 問の意図

- **b14**: [[../problems/drill/drill-b12]] は「最頻値1つ」で sort+dedup で逃げられた。複数答えにすると HashMap 集計が要る。「Map の逃げ道の限界」を体感。
- **b15**: [[../problems/drill/drill-b6]] と同じ問題を $N\le2\times10^5$ に。O(N²) は TLE → HashSet で「引きながら問い合わせ」= Map が必須な典型。
- **b16**: HashMap では無理な**範囲クエリ**。BTreeMap の `.range()` を使う。
- **b17**: 集計 ＋ 昇順出力。BTreeMap なら自動でソート済み（HashMap だと別途 sort）。HashMap と BTreeMap の差を体感。
- **b18**: 参照/deref の総合演習。`(String, i64)` のタプル Vec で `s: &String`, `p: &i64` の扱い。

## Stage 2 の所見（解き終えたら記入）

（HashMap/BTreeMap が「呪文」から「道具」になったか。残った詰まりは？ → 土曜 ABC 前の最終調整に）
