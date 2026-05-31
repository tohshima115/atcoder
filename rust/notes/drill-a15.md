---
tags:
  - rust基礎
  - match
  - 文字列型
problem: drill-a15
date: 2026-05-31
difficulty: drill
---

## 問題の要約

月（1〜12）を入力し、春・夏・秋・冬のどれかを出力する。

## 実装アプローチ

最初は `String::new()` + `mut` + `to_string()` で書いたが、`match` 式に書き直した。

```rust
let season = match m {
    3..=5 => "spring",
    6..=8 => "summer",
    9..=11 => "autumn",
    _ => "winter",
};
println!("{}", season);
```

## このセッションで学んだこと

### `&str` vs `String`

| | イメージ |
|---|---|
| `&str` | 文字列の「読み取り専用の参照」。軽い |
| `String` | 文字列の「実体を持つ箱」。変更できる |

`"hello"` と書いたリテラルは自動的に `&str`。

詰まるパターン：`String` 型の変数に `"hello"` を代入しようとしたとき。

```rust
let mut s: String = "";           // エラー：&str を String に入れられない
let mut s: String = "".to_string(); // OK（でも冗長）
```

競プロでは `match` や `if-else` 式で書けば型推論が `&str` にしてくれるので悩まない。

🟡 雰囲気理解

---

### `String::new()` を使う場面

「文字を1個ずつ追加して文字列を組み立てる」ときだけ使う。

```rust
let mut result = String::new();
for c in chars {
    result.push(c);
}
```

それ以外（値を選ぶだけ）では `match` / `if-else` 式で書けるので不要。

🟢 実装可能

---

### `match` 式とレンジパターン

`3..=5` は「3以上5以下」を表すレンジパターン。`if` の連鎖より読みやすい。

`match` は式なので戻り値を `let` で受け取れる。`mut` 不要、`to_string()` 不要。

```rust
let x = match value {
    パターン => 値,
    _ => デフォルト値,  // _ はワイルドカード（どれにも当てはまらない）
};
```

🟢 実装可能

---

### `if-else` 式

Rust の `if-else` は式なので値を返せる。

```rust
let season = if 3 <= m && m <= 5 {
    "spring"
} else {
    "winter"
};
```

`match` が使えない複雑な条件のときはこちら。

🟢 実装可能
