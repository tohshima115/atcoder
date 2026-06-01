---
tags:
  - rust基礎
  - イテレータ
  - Option型
  - unwrap
problem: drill-a21
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a21|問題文]]

N個の整数から最大値を1つだけ除いた残りの合計を出力する。N=1のとき0。

## 実装

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let ans = a.iter().sum::<i64>() - *a.iter().max().unwrap();
    println!("{}", ans);
}
```

「全体の合計 - 最大値」で求める。最大値が複数あっても1個だけ引かれるので仕様と一致。N=1のときは `sum == max` で答えは0になる。

**最初の失敗**: `sum::<&i64>()` と `a.iter().max()` をそのまま引き算しようとした。型が合わないのでコンパイルエラー。

---

## 学んだこと

### Option型 — 「値があるかもしれない/ないかもしれない」を表す型

Rustには `null` がない。代わりに `Option<T>` という型で「あるかも/ないかも」をコンパイル時に表現する。

```rust
Some(5)   // 値がある
None      // 値がない
```

`.max()` が `Option` を返す理由：空の配列に最大値は存在しないため。

```rust
let empty: Vec<i64> = vec![];
empty.iter().max()  // None を返す（panicしない）
```

知識レベル: 🟡 雰囲気理解

---

### `.unwrap()` — Option の箱を強制的に開ける

```rust
let x: Option<i64> = Some(5);
x.unwrap()  // → 5

let y: Option<i64> = None;
y.unwrap()  // → panic！
```

- `Some` なら中身を取り出す
- `None` なら実行時 panic
- **「None になりえない保証がある場面」でだけ使う**

競プロでは制約上 `N >= 1` が保証されるので `.max().unwrap()` は安全。

`.unwrap()` は数値専用ではなく、`Option<T>` の `T` が何でも使える。

```rust
let s: Option<&str> = Some("hello");
s.unwrap()  // &str が出てくる
```

知識レベル: 🟡 雰囲気理解

---

### `*` デリファレンス — 参照から値を取り出す

`.iter().max()` は `Option<&i64>`（参照が入った Option）を返す。`.unwrap()` で開けると `&i64`（参照）。計算に使うには `i64`（値）が必要なので `*` で外す。

```rust
*a.iter().max().unwrap()
// ① .max()     → Option<&i64>
// ② .unwrap()  → &i64
// ③ *          → i64
```

知識レベル: 🟡 雰囲気理解

---

### 現場での Option の扱い（参考）

競プロでは `unwrap()` だらけでも問題ないが、現場コードでは panic しないよう別の書き方をする。

```rust
v.iter().max().unwrap_or(&0)        // Noneなら0を使う

if let Some(max) = v.iter().max() { // Someのときだけ処理
    println!("{}", max);
}

v.iter().max()?   // 関数内でNoneなら即return None（? 演算子）
```

知識レベル: 🟡 雰囲気理解（競プロでは今は不要）
