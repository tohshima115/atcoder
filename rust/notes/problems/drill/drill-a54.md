---
tags:
  - rust基礎
  - 文字列
  - filter
  - count
  - itertools
  - Display
  - Debug
problem: drill-a54
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a54|問題文]]

英小文字のみの文字列 S に含まれる文字 C の個数を出力する。

## 実装アプローチ

**最短解**:
```rust
use proconio::input;

fn main() {
    input! {
        s: String,
        c: char,
    }
    let cnt = s.chars().filter(|&ch| ch == c).count();
    println!("{}", cnt);
}
```

最初は `itertools::counts_by` で全部数えようとしてオーバーキル（しかも引数が「キー関数」なので使い方が違ってた）。

---

## 学んだこと

### 「1種類を数える」 → `filter().count()`

```rust
let cnt = s.chars().filter(|&ch| ch == c).count();
```

- `chars()` は `char` を値で返すので filter のクロージャは `|&ch|`（参照1枚外し）
- `count()` は `usize` を返す

「特定の値が何回出るか」だけ知りたいなら **全要素を数える必要はない**。`filter + count` が最短。

知識レベル: 🔵 説明可能

---

### 「全種類数える」 → HashMap or itertools::counts

```rust
// 標準ライブラリ
use std::collections::HashMap;
let mut m: HashMap<char, i64> = HashMap::new();
for c in s.chars() {
    *m.entry(c).or_insert(0) += 1;
}

// itertools
use itertools::Itertools;
let m: HashMap<char, usize> = s.chars().counts();
```

各文字が何回出たかを全部知りたい時はこちら。**B 問題以降で頻出**。今回みたいに1種類だけ調べる場合はやりすぎ。

知識レベル: 🟡 雰囲気理解

---

### `itertools::counts_by` の引数は「キー関数」

```rust
// ❌ こうじゃない（c は char、関数じゃない）
s.chars().counts_by(c)

// ✅ こう（分類のためのキー関数を渡す）
s.chars().counts_by(|c| c.is_ascii_lowercase())
// → HashMap<bool, usize> : 小文字何個、それ以外何個
```

- `counts_by(f)` は「**`f(要素)` の結果でグループ化** して数える」
- `counts()` （引数なし）は「**要素そのもの** でグループ化」

メソッド名から「`x` を数える」ように見えるが、実際は「`f(x)` でグループ分けして数える」ので注意。

知識レベル: 🟡 雰囲気理解

---

### `Display` (`{}`) と `Debug` (`{:?}`) の違い

```rust
println!("{}", count);     // ← HashMap は Display 未実装 → コンパイルエラー
println!("{:?}", count);   // ← Debug は実装してるので OK
```

| フォーマット | トレイト | 用途 |
|---|---|---|
| `{}` | `Display` | **人間向けの表示**（プリミティブ型、`String`、自分で実装した型） |
| `{:?}` | `Debug` | **デバッグ用の表示**（`Vec`, `HashMap`, タプル、自分で実装した型） |
| `{:#?}` | `Debug`（pretty） | インデント付きの整形版 |

- プリミティブ型（`i64`, `&str`, `char`...）は両方実装してる
- コレクション（`Vec`, `HashMap`, `Tuple`）は **Debug のみ**
- AtCoder の出力は基本「人間向け」なので `{}` で書くもの。デバッグ時だけ `{:?}` で覗く

知識レベル: 🔵 説明可能

---

### 使い分けの判断基準

| 状況 | 使うもの |
|---|---|
| 1種類だけ数える | `filter().count()` |
| 全種類数える（少数） | `HashMap` + `entry().or_insert(0) += 1` |
| 全種類数える（ワンライナー） | `itertools::counts()` |
| 条件でグループ化して数える | `itertools::counts_by(f)` |

「やりたいことを最小手数でやる」のが大事。問題に合わない大砲を持ち出すと、型パズルにハマって時間を溶かす。

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a48]] — `HashMap` + `entry().or_insert(0) += 1` の出現回数
- [[drill-a42]] — `chars()` + `map` で1文字ずつ処理
- [[drill-a30]] — `filter` で `|&&x|`（こちらは `iter()`、`chars()` の `|&ch|` と対比）
- [[ref-hashmap-hashset]] — HashMap/HashSet のまとめ
