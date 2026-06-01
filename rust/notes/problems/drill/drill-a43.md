---
tags:
  - rust基礎
  - proconio
  - クエリ
  - イテレータ
  - タプル
  - パターンマッチ
problem: drill-a43
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a43|問題文]]

長さ N の数列 A（初期値0）に対し、Q 個のクエリ `(X_i, V_i)` で「A[X_i] に V_i を加える」操作を全て行った後の数列を出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, i64); q],
    }
    let mut a: Vec<i64> = vec![0i64; n];
    for (x, v) in queries {
        a[x - 1] += v;
    }
    let ans: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
```

`queries` をそのまま `for` で消費する書き方が一番素直。

---

## 学んだこと

### proconio でクエリ配列を受け取る

```rust
input! {
    n: usize, q: usize,
    queries: [(usize, i64); q],
}
```

- `[型; 個数]` の中にタプル型 `(T1, T2)` を書ける
- 1行に複数値が並ぶクエリ系問題で頻出
- 3要素以上のクエリでも同様: `[(usize, usize, i64); q]`

知識レベル: 🟢 実装可能

---

### `vec![0i64; n]` — 0で初期化した配列

```rust
let mut a: Vec<i64> = vec![0i64; n];
```

- `vec![値; 個数]` で「指定の値で埋めた Vec」を作る
- 型を明示したいときは `0i64` のようにリテラルにサフィックスを付ける（または型注釈）

知識レベル: 🟢 実装可能

---

### `for (x, v) in queries` がなぜ動くか — 消費系イテレート

```rust
for (x, v) in queries {
    a[x - 1] += v;
}
```

- `for ... in queries` は **`queries.into_iter()` を暗黙に呼ぶ**
- 取り出される要素は `(usize, i64)`（タプルが値で来る）
- `(x, v)` でパターンマッチして `x: usize, v: i64` に分解
- **`queries` は消費されて使えなくなる**（あとで使うなら `&queries` で借りる）

知識レベル: 🔵 説明可能

---

### クエリループの書き方4パターン

```rust
// ① 消費する（一番楽。あとで queries 使わないならこれ）
for (x, v) in queries { ... }
// x: usize, v: i64

// ② 借りる + 参照のまま受ける
for (x, v) in &queries { ... }
// x: &usize, v: &i64 → 使うとき *x が必要

// ③ タプル全体の & を1個外す
for &(x, v) in &queries { ... }
// x: usize, v: i64

// ④ 中身の & を1個ずつ外す
for (&x, &v) in &queries { ... }
// x: usize, v: i64
```

**③ と ④ は等価**。「タプルの参照 `&(A, B)`」と「参照のタプル `(&A, &B)`」は Rust のパターンマッチ的に行き来できる。コンパイラが両方解釈してくれる。

**判断基準**:
- もう使わない → ① で消費
- あとで使う → ③ か ④（書きやすい方）

知識レベル: 🔵 説明可能

---

### `for &v in &a` のパターン（既出の復習）

```rust
let a: Vec<i64> = vec![...];
for &v in &a {
    println!("{}", v);
}
```

- `&a` で借りる（消費しない）
- イテレータの要素は `&i64`
- `&v` でパターンマッチして値 `v: i64` を取り出す

「借りたい + 値で扱いたい」ときの定番。今回のクエリの ③ ④ もこれの応用。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-a36]] — 1-indexed → 0-indexed の `x - 1`
- [[drill-a30]] — `|&&x|` で参照を二重に外す（filter の場合）
- [[drill-a41]] — `iter()` が参照を返す話、`.copied()` の使い分け
