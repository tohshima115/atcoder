---
tags:
  - topic
  - match
  - if式
  - 網羅性
  - unreachable
type: topic
date: 2026-06-01
---

# match の網羅性・if式・unreachable

> **繰り返し課題**: `match` にいつ `_ =>` が要るのか、到達しない `else` に `unreachable!()` を入れる羽目になる、`if`/`match` が式だと忘れる。
> 出てきた問題: [[drill-a15]] [[drill-a18]] [[drill-a44]] [[drill-a52]]

## `if` も `match` も「式」= 値を返す

```rust
let season = match m {          // let の右辺に置ける
    3..=5 => "spring",
    6..=8 => "summer",
    _     => "winter",          // _ はワイルドカード
};
println!("{}", if cond { "Yes" } else { "No" });  // println の中にも書ける
```

- ブロック最後の式が値（**`;` を付けない**のが肝）
- 全アーム / `if`・`else` 両方の **型が揃う**必要あり
- `mut` も `to_string()` も要らなくなる（[[drill-a15]]）

レンジパターン `3..=5` で if 連鎖より読みやすく（[[drill-a15]]）。

## `_ =>` が要るか早見表

| 型 | バリアント | `_ =>` |
|---|---|---|
| `Option<T>` | 2 (Some/None) | 全部書けば不要 |
| `Result<T,E>` | 2 (Ok/Err) | 全部書けば不要 |
| `Ordering` | 3 | 全部書けば不要（[[drill-a44]]） |
| `bool` | 2 | 全部書けば不要 |
| 整数・String・&str | 無限 | **必須** |

→ enum を網羅すれば `_` は不要。Option の処理は [[option-result-handling]]、Ordering は [[sort-cmp-ordering]]。

## 「到達しない else」は設計のサイン

整数の全順序は `>` `<` `==` で尽きるので、3つ目に `else if a == b` は不要:

```rust
if a > b { ... } else if a < b { ... } else { /* 必ず a == b */ }
```

ここに `unreachable!()` を入れたくなったら **`match a.cmp(&b)` に寄せる**合図（[[drill-a44]]）。網羅 `match` ならコンパイラが全ケースを保証してくれる。

## `unreachable!()` の作法

```rust
match x {
    "rock" => 0, "scissors" => 1, "paper" => 2,
    _ => unreachable!(),   // マクロ。! は後ろ。never型(!) を返す
}
```

`!unreachable()` は誤り（[[drill-a18]] [[drill-a44]]）。`&str`/整数の match は無限バリアントなので `_` が必須、そこに「来ないはず」を明示するのが `unreachable!()`。

## if/else vs match 使い分け

| 状況 | おすすめ |
|---|---|
| 値の比較・複雑な論理式 | `if/else` |
| enum / 固定パターンで分岐 | `match` |
| 全網羅を保証したい | `match`（コンパイラチェック） |
| 3値以上の排他分岐 | `match` |

enum を返す API（`Option`/`Result`/`Ordering`）を見たら反射で `match`。

## 関連

- [[option-result-handling]] — Option/Result の match
- [[sort-cmp-ordering]] — `cmp` と `Ordering`
- [[string-char-conversion]] — `match c { 'a'|'e' => ... }` の母音判定
