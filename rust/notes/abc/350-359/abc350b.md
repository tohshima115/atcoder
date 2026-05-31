---
tags:
  - rust基礎
  - vec
  - usize
  - bool
  - xor
problem: abc350b
date: 2026-05-31
difficulty: abc-B
---

## 問題へのリンク

[ABC350 B - Dentist Aoki](https://atcoder.jp/contests/abc350/tasks/abc350_b)

## 問題の要約

N本の歯に対してQ回の治療（穴Tiを指定）を行う。治療は「歯があれば抜く、なければ生やす」トグル操作。最終的な歯の本数を答える。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }
    let mut teeth = vec![true; n + 1];
    for i in 0..q {
        teeth[t[i]] ^= true;
    }
    let mut ans = 0;
    for i in 1..=n {
        if teeth[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
```

- `vec![true; n+1]` で全歯あり状態を初期化（添字0は使わない）
- `^= true` でトグル（XOR）
- 最後に `true` の個数を数える

## このセッションで学んだこと

### `vec!` マクロ

`!` がついているのはマクロ（`println!` と同じ仲間）。`vec![値; 個数]` で同じ値を並べたVecを作る。

```rust
vec![true; 5]  // [true, true, true, true, true]
vec![0; 3]     // [0, 0, 0]
```

知識レベル: 🟢 実装可能

---

### 添字（インデックス）と `usize`

「添字」= 配列の何番目かを指定する数（`teeth[3]` の `3` の部分）。

Rustでは配列の添字は必ず `usize` 型でないといけない。`i64` を使うとコンパイルエラーになる。

```rust
let v = vec![0; 5];
let i: i64 = 2;
v[i];          // エラー: i64 は添字に使えない
v[i as usize]; // OK: キャストすればよい
```

proconio で読む時点から `usize` にしておくのが楽：

```rust
input! {
    n: usize,
    t: [usize; q],
}
```

**`usize` を使ってはいけないケース：** 引き算で負になる可能性がある場合。`usize` は0未満になれないのでパニックする。

```rust
let a: usize = 3;
let b: usize = 5;
let c = a - b;  // パニック！
```

| 用途 | 型 |
|---|---|
| 配列の添字、Vec のサイズ | `usize` |
| 計算で負になりうる値 | `i64` |
| 迷ったら | `i64` |

知識レベル: 🟡 雰囲気理解

---

### `bool` の XOR トグル `^= true`

`^=` は XOR 代入演算子。`bool` に使うと値を反転（トグル）できる。

```rust
let mut x = true;
x ^= true;  // false
x ^= true;  // true（元に戻る）
```

今回の問題のように「あれば抜く、なければ生やす」というトグル操作に使える。

知識レベル: 🟢 実装可能

---

### `for` ループの2パターン

**パターン1：範囲でループ（添字を使う）**
```rust
for i in 0..n {
    println!("{}", v[i]);  // i は usize
}
```

**パターン2：要素を直接取り出す（参照）**
```rust
for x in &v {
    println!("{}", x);  // x は &T（参照）
}
```

添字が必要なときはパターン1、要素だけ使うときはパターン2。
`&v` の `&` は「借りる」という意味で、参照の概念（詳細は後で）。

知識レベル: 🟡 雰囲気理解
