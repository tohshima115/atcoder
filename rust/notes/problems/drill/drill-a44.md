---
tags:
  - rust基礎
  - if式
  - match
  - cmp
  - Ordering
  - unreachable
problem: drill-a44
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a44|問題文]]

2つの整数 A, B を比較して `Greater` / `Less` / `Equal` を出力する。

## 実装アプローチ

**素直な if/else 版**:
```rust
use proconio::input;

fn main() {
    input! { a: i64, b: i64 }
    let ans = if a > b {
        "Greater"
    } else if a < b {
        "Less"
    } else {
        "Equal"
    };
    println!("{}", ans);
}
```

**Rust らしい cmp + match 版（推奨）**:
```rust
use std::cmp::Ordering;
use proconio::input;

fn main() {
    input! { a: i64, b: i64 }
    let ans = match a.cmp(&b) {
        Ordering::Greater => "Greater",
        Ordering::Less    => "Less",
        Ordering::Equal   => "Equal",
    };
    println!("{}", ans);
}
```

最初に書いたコードでは3つ目の `else if a == b` の後に `else { !unleachble() }` を入れていた → 整数の全順序では `a > b` でも `a < b` でもない時は必ず `a == b` なので不要。タイポも複数あった（`unleachble` → `unreachable`、`!unreachable()` → `unreachable!()`）。

---

## 学んだこと

### 排他的な3分岐は `else if` を1個減らせる

整数のような **全順序を持つ型** では `>`, `<`, `==` の3つで全パターンを尽くす。なので:

```rust
if a > b { ... }
else if a < b { ... }
else { /* ここは必ず a == b */ }
```

`else if a == b` を書く必要はない。書くと**到達しない `else` ブランチ**が生まれて、そこに `unreachable!()` を入れる羽目になる。これは設計のミスサイン。

知識レベル: 🔵 説明可能

---

### `unreachable!()` の書き方と意味

```rust
unreachable!()    // マクロ。! はマクロ呼び出しの ! で、後ろに付く
```

- 「ここには絶対到達しない」ことを宣言するマクロ
- 戻り値の型は **`!`（never型）** — 値を返さない特殊な型
- ランタイムに到達した場合は panic する

**注意点**:
- `!` を前に付けるとタプルの否定演算と紛らわしい。マクロは `unreachable!()` と後ろに `!`
- そもそも到達不能を保証できるなら `match` で網羅した方が安全

知識レベル: 🟡 雰囲気理解

---

### `a.cmp(&b)` と `Ordering`

```rust
use std::cmp::Ordering;
let o: Ordering = a.cmp(&b);
```

- `cmp` は **`Ordering`**（`Greater` / `Less` / `Equal` の3値 enum）を返す
- 引数は **参照** `&b`（`Ord` trait の `cmp` シグネチャがそうなっている）
- `match` で網羅すると `unreachable!()` 不要 — コンパイラが「全ケース書いたか」を保証してくれる

```rust
match a.cmp(&b) {
    Ordering::Greater => "Greater",
    Ordering::Less    => "Less",
    Ordering::Equal   => "Equal",
}
```

問題文の3値出力にハマりすぎな API。a44 の専用 API レベル。

知識レベル: 🔵 説明可能

---

### `if` も `match` も式 — 値を返せる

```rust
let ans = if a > b { "Greater" } else { ... };
let ans = match a.cmp(&b) { ... };
```

- どちらも **式**（値を返す）なので `let` の右辺に書ける
- ブロックの最後の式が値（`;` を付けないことが重要）
- 全アームの型が揃ってないとエラー（`if` なら `if/else` 両方、`match` なら全アーム）

知識レベル: 🟢 実装可能

---

## 「if/else vs match」の使い分け

| 状況 | おすすめ |
|---|---|
| 条件式が値の比較・複雑な論理式 | `if/else` |
| enum や値の固定パターンで分岐 | `match` |
| 全パターン網羅を保証したい | `match`（コンパイラチェックされる） |
| 3値以上の排他分岐 | `match`（読みやすい） |

`Ordering`, `Option`, `Result` など enum を返す API を扱うときは反射的に `match`。

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a42]] — `if` を `map` の中で式として使うパターン
- [[drill-a41]] — `if ans { "Yes" } else { "No" }` の Yes/No 出力
