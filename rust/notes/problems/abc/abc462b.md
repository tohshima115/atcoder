---
tags:
  - rust基礎
  - proconio
  - 入力
  - ジャグ配列
  - 隣接リスト
problem: abc462b
date: 2026-06-13
difficulty: abc-B
---

## 問題へのリンク

[ABC462 B - Gift](https://atcoder.jp/contests/abc462/tasks/abc462_b)

## 問題の要約

人 i が贈った相手リストが行ごとに与えられる（行の先頭 `K_i` が個数）。これを反転して「人 i は誰から贈られたか」を昇順で各行に出力する。

## 詰まったポイント

ロジックは簡単（リストの反転）だったのに、**入力が「行ごとに個数が変わるジャグ配列」**で、proconio での読み方がわからず詰んだ。アルゴリズム力ではなく実装メカニクスの穴。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! { n: usize }

    let mut recv: Vec<Vec<usize>> = vec![vec![]; n + 1]; // 1-indexed

    for i in 1..=n {
        input! {
            k: usize,
            a: [usize; k],   // 直前に読んだ k を長さに使える
        }
        for x in a {
            recv[x].push(i); // 「x は i から贈られた」
        }
    }

    for i in 1..=n {
        let b = &recv[i];
        print!("{}", b.len());
        for x in b {
            print!(" {}", x);
        }
        println!();
    }
}
```

## このセッションで学んだこと

### ジャグ配列（行ごとに個数が違う入力）の読み方 = 2段読み

「先頭の数 = この後に何個続くか」というタイプ。**まず個数 `k` を読む → その `k` を使って長さ `k` の配列を読む** の2段構え。

ポイントは2つ:

1. **`input!` は何回でも呼べて、呼ぶたびに続きから読む。** だからループの中で呼べばよい。「N行まとめて2次元配列で読もう」とすると長さがバラバラで表現できず詰む。**行ごとに read を分ける**のが鍵。
2. **`[usize; k]` の長さに、直前に読んだ変数 `k` を使える。**

```rust
input! { n: usize }
for i in 1..=n {
    input! {
        k: usize,
        a: [usize; k],   // ← k は同じ行のさっき読んだ値
    }
}
```

このパターンは ABC の B〜C で頻出（隣接リスト・グラフ入力など）。手に馴染ませておくと得。

知識レベル: 🟢 実装可能

---

### 隣接リスト風の反転（push で逆向きを作る）

入力は「i → 相手」だが欲しいのは「i ← 誰」。送り主 `i` を相手 `x` のリストに push していけば反転できる。

```rust
let mut recv: Vec<Vec<usize>> = vec![vec![]; n + 1];
for i in 1..=n {
    for x in a {
        recv[x].push(i);
    }
}
```

地味に嬉しいのは、**送り主 `i` を 1→N の順で処理しているので `recv[x]` には自然に昇順で積まれる**こと。出力は昇順要求だが、別途ソート不要。

知識レベル: 🟢 実装可能

---

### Vec<Vec<T>> の初期化と 1-indexed

`vec![vec![]; n + 1]` で空 Vec を n+1 個並べる。添字0を捨てて 1..=n を使うと問題の番号とそろって楽（[[abc350b]] の `vec![true; n+1]` と同じ発想）。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[abc462d]] — 同じコンテストの D（imos + 組み合わせ）
- 入力まわりで詰まったら [[competitive-rust-index]]
