# 学習ノート 索引

AtCoder 精進の学習ノート。**種類別**に整理している。

```
notes/
├── topics/      … 繰り返し出てくる課題のテーマ別まとめ（横断）
├── problems/    … 問題ごとの記録
│   ├── drill/   … ドリル a3〜a54
│   ├── tessoku/ … 鉄則本 a/b
│   └── abc/     … ABC 本番
├── reference/   … チートシート（脳内に入れておくもの）
└── drills/      … 練習問題セット
```

> Obsidian の `[[ ]]` リンクはファイル名で解決されるので、フォルダを移動してもノート間リンクは生きる。

---

## 🔁 topics — 繰り返し課題まとめ

「個々の問題で何度も引っかかったこと」を横断でまとめたもの。**詰まったらまずここ**。

> 🔎 **入口は [[competitive-rust-index]]（やりたいこと→書き方の逆引き辞書）**。本番で迷ったらまずこれを引く。

| テーマ | 内容 |
|---|---|
| [[sum-collect-turbofish]] | `sum`/`collect` の型推論・ターボフィッシュ |
| [[option-result-handling]] | `Option`/`Result`・`unwrap`・`map_or`・`filter_map` |
| [[integer-types-overflow]] | 型選び・$10^9$ ルール・整数除算・切り上げ |
| [[string-char-conversion]] | `String`↔`Vec<char>`↔数値、UTF-8、`join` vs `collect` |
| [[iterator-catalog]] | イテレータ操作カタログ（map/filter/find/windows…） |
| [[sort-cmp-ordering]] | ソート・`cmp`/`Ordering`・破壊的 Vec 操作 |
| [[match-exhaustiveness]] | `match` 網羅性・if式・`unreachable!()` |
| [[references-deref]] | 参照と参照外し（`&&`/`*`/`copied`/クロージャ） |

---

## 📚 reference

- [[hashmap-hashset]] — `HashMap` / `HashSet` 頻出操作まとめ

## 🗒 drills

- [[monday-drill]] — AB完答のための頻出パターン9問
- [[2d-grid-drill]] — 2次元グリッド克服ドリル（単純操作・累積和・文字列変換・頻出13問）

---

## 📝 problems — 問題ごとのノート

### drill
[[drill-a3]] · [[drill-a4]] · [[drill-a5-a14]] · [[drill-a15]] · [[drill-a18]] · [[drill-a20]] · [[drill-a21]] · [[drill-a22]] · [[drill-a23]] · [[drill-a24]] · [[drill-a26]] · [[drill-a28]] · [[drill-a29]] · [[drill-a30]] · [[drill-a35]] · [[drill-a36]] · [[drill-a37]] · [[drill-a38]] · [[drill-a39]] · [[drill-a41]] · [[drill-a42]] · [[drill-a43]] · [[drill-a44]] · [[drill-a45]] · [[drill-a46]] · [[drill-a47]] · [[drill-a48]] · [[drill-a49]] · [[drill-a50]] · [[drill-a51]] · [[drill-a52]] · [[drill-a53]] · [[drill-a54]]

### tessoku
[[tessoku-a03]] · [[tessoku-a04]] · [[tessoku-b03]] · [[tessoku-b04]]

### abc
[[abc350a]] · [[abc350b]] · [[abc353a]]

---

## 運用メモ

- 新しい問題ノートは `problems/{drill,tessoku,abc}/` に置く（`save-learning` スキル）
- 同じ課題に **3回目**でつまずいたら、`topics/` の該当ファイルに追記するか新規テーマを起こす
- 各問題ノートの「関連ノート」から `topics/` へ逆リンクを張ると探しやすい
