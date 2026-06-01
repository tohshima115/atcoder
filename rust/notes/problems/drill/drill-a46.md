---
tags:
  - rust基礎
  - 文字列
  - スライス
  - UTF-8
  - 1-indexed
  - collect
problem: drill-a46
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a46|問題文]]

英小文字のみの文字列 S の L 文字目から R 文字目までを取り出して出力する。

## 実装アプローチ

**ASCII限定で最短版**:
```rust
use proconio::input;

fn main() {
    input! {
        s: String,
        l: usize,
        r: usize,
    }
    println!("{}", &s[l-1..r]);
}
```

**汎用版（Vec<char> 経由）**:
```rust
let c: Vec<char> = s.chars().collect();
let ans: String = c[l-1..r].iter().collect();
println!("{}", ans);
```

最初に書いたコードは `Vec<char>` に `collect` してから `join()` を呼ぼうとしていた → どちらも誤り。

---

## 学んだこと

### `&s[a..b]` は **バイト** インデックスのスライス

```rust
let sub = &s[l-1..r];   // バイト l-1 から r-1 まで（r を含まない）
```

- 文字数ではなく **バイト数** で切る
- ASCII（1バイト文字）だけなら文字数と一致するので問題ない
- 日本語など複数バイト文字が混じると **文字の途中で切ろうとして panic**

```
"あいう" → "あ" は3バイト
&s[1..2] → panic（あ の途中）
```

知識レベル: 🔵 説明可能

---

### 「ASCII限定」と「Unicode対応」の使い分け

| 条件 | 推奨 | 例 |
|---|---|---|
| 制約に「英小文字のみ」「英大小文字のみ」 | `&s[l-1..r]` | 速い・短い |
| 日本語などUnicodeあり | `Vec<char>` 経由 | 安全 |

問題文の制約をまず見て判断する。AtCoder の A 問題はだいたい ASCII 限定。

知識レベル: 🟢 実装可能

---

### `String` への collect は `&char` でも `char` でもOK

```rust
let c: Vec<char> = s.chars().collect();
let ans: String = c[l-1..r].iter().collect();   // &char をそのまま collect
let ans: String = c[l-1..r].iter().copied().collect();  // char にしてから（こっちでもOK）
```

`String` は `FromIterator<char>` も `FromIterator<&char>` も実装してるので両方通る。`.copied()` 不要だが、付けても害はない（[[drill-a41]] の話）。

知識レベル: 🟢 実装可能

---

### `join` は `Vec<char>` には使えない

```rust
let v: Vec<char> = vec!['a', 'b'];
v.join("");   // ← コンパイルエラー
```

`join` は **文字列スライスのコレクション** 用（`Vec<String>`, `&[&str]`）。`char` 列を `String` にしたいなら `.collect::<String>()`。

| 元の型 | 文字列化 |
|---|---|
| `Vec<String>` / `&[&str]` | `.join("")` |
| `Vec<char>` / `chars()` | `.collect::<String>()` |

[[drill-a35]] の `join` と対比して覚える。

知識レベル: 🔵 説明可能

---

### 区間スライスの 1-indexed → 0-indexed

問題文「L 文字目から R 文字目まで（両端含む）」:

| 問題文 | スライス |
|---|---|
| L 文字目（1-indexed）| インデックス `l-1` |
| R 文字目（1-indexed）| インデックス `r-1` |
| L〜R 文字目（含む）| `s[l-1..r]` |

Rust の `..r` は r を含まないので、`l-1..r` でちょうど L〜R 文字目が取れる。[[drill-a36]] の区間反転と同じ。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-a35]] — `Vec<String>` の `join`（こちらは使える）
- [[drill-a36]] — 区間 `l-1..r` の同じパターン
- [[drill-a39]] — `String` と `Vec<char>` の往復、UTF-8 の話
- [[drill-a41]] — `chars()` が値を返すので `.copied()` 不要
