---
tags:
  - 道具選び
  - 計算量
  - min
  - sort
  - String
  - clone
  - 参照
  - rust基礎
problem: drill-b18
date: 2026-06-04
difficulty: drill
---

## 問題

[[../../../contests/drill/b18|問題文]]

$N$ 人の `(名前 s, 得点 p)`。最高得点者の名前を出力。同点なら**名前が辞書順最小**。

## 実装アプローチ

**BTreeMap を使おうとして詰まった**回。これは範囲クエリの問題ではなく、**map を使わず Vec を素直に回す**のが正解。

**方式A：単一ループで best を追う**
```rust
use proconio::input;
fn main() {
    input! { n: usize, student: [(String, i64); n] }
    let mut best_score: i64 = -1;
    let mut best_name = String::new();
    for (name, score) in &student {        // name: &String, score: &i64
        if *score > best_score || (*score == best_score && *name < best_name) {
            best_score = *score;
            best_name = name.clone();       // String は Copy でないので clone
        }
    }
    println!("{}", best_name);
}
```

**方式B：最大求める → 絞る → 最小（読みやすい）**
```rust
let best = student.iter().map(|(_, s)| *s).max().unwrap();   // O(N)
let ans = student.iter()
    .filter(|(_, s)| *s == best)   // 最高得点の人だけ
    .map(|(name, _)| name)         // 名前だけ
    .min()                          // 辞書順最小（sort 不要！）
    .unwrap();
```

---

## 学んだこと

### `range` はキー専用 → 「値で探す」には BTreeMap は不向き

最初 `BTreeMap<String, i64>`（キー=名前）を作って `range(max..)` しようとしたが、
**`range` は必ずキーにかかる**ので「名前」に対してかかってしまい、得点（値）では探せない。
得点をキーにしても、**同点はキー重複で潰れる**うえ「名前最小」の情報が消える。

→ **教訓：探したいものをキーにできないなら map は向かない。** この問題は
「最大を探して同点は別基準」なので、**Vec を素直に 1 回なめる**のが一番きれい。
「BTreeMap だから range」ではなく**問題に合う道具を選ぶ**（→ [[drill-b16]] は時刻がキーだから range が効いた）。

知識レベル: 🔵 説明可能

### 「並べて先頭」は過剰、最小1個なら `min`（[[drill-b3]] の再確認）

「BestScore のリストを作る → **昇順 sort → 先頭**」でも正しいが、計算量が増える:

| 方式 | 計算量 |
|---|---|
| 単一ループ（best 追跡） | O(N) |
| 二段階 ＋ **sort して先頭** | O(N + m log m) ≈ O(N log N) |
| 二段階 ＋ **min で取る** | **O(N)**（sort を min に変えるだけ） |

「昇順に並べて先頭」＝「最小が欲しい」だけ。**最小1個に全ソートは過剰**で、`min()` なら O(m)。
→ [[drill-b3]] の「K番目1個に全ソートは過剰」と同じ話。「並べて端を取る」と書きたくなったら
**「min/max で1個取れないか」**を考える。

（※今回 N≤2×10⁵ なので O(N log N) でも余裕で AC。AC目的なら sort でも可。詰める意識として min）

知識レベル: 🔵 説明可能

### `String` は Copy でないので保存には `clone`

ループ変数 `name: &String` から値を取っておくには `name.clone()` が要る（`i64` の `score` は
`*score` でコピーされるが、`String` はコピーされない）。`Copy` と `Clone` の違いの実例。

知識レベル: 🟢 実装可能

---

## このセッションの総括

| 学び | 要点 |
|---|---|
| 道具選び | range はキー専用。値で探す/同点処理は Vec を素直に回す |
| min vs sort | 最小1個なら sort（O(N log N)）でなく min（O(N)） |
| String | Copy でないので保存に clone |

**学び**: 直前に BTreeMap をやったからといって全部 map ではない。**問題に合う道具を選ぶ**判断こそが本題だった。

---

## 関連ノート

- [[drill-b16]] — range が効く例（キー=探したいもの）との対比
- [[drill-b3]] — 「1個だけ欲しいなら全ソートは過剰」（min/select の話）
- [[../../topics/references-deref]] — `&String`/`*name` とタプルの参照
- [[../../topics/string-char-conversion]] — String の扱い
