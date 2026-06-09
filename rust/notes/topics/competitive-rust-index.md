---
tags:
  - 逆引き
  - index
  - reference
date: 2026-06-06
---

# 競プロRust 逆引きインデックス（やりたいこと → どこを見る）

> **本番で「これどう書くんだっけ?」となったらまずここ。**
> 市販の競プロ×Rust辞書はどれも穴だらけだったので、**自分が実際に詰まった所だけ**を集めた専用辞書として育てる。
> ✅ = ノート有り ／ 🕳 = まだ無い（詰まったら起こす）

---

## 入出力
- 🕳 `proconio::input!` の型指定（`usize` / `i64` / `[T; n]` / `String` / `chars`）→ [[io-proconio]]
- 🕳 1行を文字配列で受ける・空白区切りで受ける
- ✅ 数字⇄文字・`String`⇄`Vec<char>` → [[string-char-conversion]]

## Vec・配列
- 🕳 1次元の初期化 `vec![0; n]`、`push`/`pop`/index
- ✅ **2次元配列**：生成 `vec![vec![0; w]; h]`、`Vec<String>→Vec<Vec<_>>`、走査 → [[2d-grid-and-prefix]]
- ✅ ソート・逆順・`cmp`/`Ordering`・破壊的操作 → [[sort-cmp-ordering]]
- ✅ イテレータ操作カタログ（map/filter/find/windows/fold…）→ [[iterator-catalog]]
- ✅ `collect`/`sum` の型推論・ターボフィッシュ → [[sum-collect-turbofish]]

## 文字列
- ✅ `String`↔`Vec<char>`↔数値・UTF-8・`join` vs `collect` → [[string-char-conversion]]
- 🕳 `bytes()` と `b'0'` 引き算で数字化、`as_bytes()[i]`

## マップ・集合
- ✅ `HashMap` / `HashSet` 頻出操作 → [[hashmap-hashset]]
- ✅ `HashMap` か `BTreeMap` か・range → [[map-when-and-btreemap]]
- ✅ `entry().or_default()` でカウント → [[hashmap-hashset]]

## 数値・型
- ✅ 型選び（usize/i64/i32）・10⁹ルール・整数除算・切り上げ → [[integer-types-overflow]]
- ✅ オーバーフロー（型の壁 10⁹/10¹⁸）→ [[integer-types-overflow]]
- ✅ bit演算・bit全探索 → [[bit-operations]]

## 参照・所有権
- ✅ 参照と参照外し（`&&`/`*`/`copied`/クロージャ内）→ [[references-deref]]

## 制御フロー
- ✅ `match` 網羅性・if式・`unreachable!()` → [[match-exhaustiveness]]
- ✅ `Option`/`Result`・`unwrap`・`map_or`・`filter_map` → [[option-result-handling]]

---

## アルゴリズム道具箱（緑への必須セット）
> ここが今の主戦場。空欄を埋めるたびに緑が近づく。

- 🕳 **累積和（1次元）** + 「和=K の数え方」（seen so far / two-sum）→ [[2d-grid-and-prefix]] / [[drill-b15]]
- ✅ **2次元累積和・縦累積和** → [[2d-grid-and-prefix]]
- 🕳 **2次元 imos** → [[2d-grid-and-prefix]]
- 🕳 **二分探索**（`binary_search` / めぐる式 / 答えで二分）→ [[binary-search]]
- 🕳 **DP（ナップザック・配るDP/もらうDP）** → [[dp-basics]]
- 🕳 **BFS / DFS・グラフ表現**（隣接リスト・VecDeque）→ [[graph-basics]]
- 🕳 **貪欲の交換論法**（abc461 C：色リーダー枠＋自由枠の分離）→ [[greedy-exchange]]

## 計算量の判断
- ✅ 逆引き表（N → 許される計算量）・GO/NO-GO儀式・「足し算でなく最大項」→ [[complexity-estimation]]

---

## 運用
- 本番/精進で詰まった機構は、まずここに🕳行を足す → 後でノート本体を起こして✅に
- 既存 topics への入口はここに集約。**詰まったら市販辞書より先にここを引く**
- 関連：[[../README]]（ノート全体の索引）
