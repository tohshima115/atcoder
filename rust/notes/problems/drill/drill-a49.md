---
tags:
  - rust基礎
  - イテレータ
  - sum
  - max
  - ターボフィッシュ
problem: drill-a49
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a49|問題文]]

長さ N の数列 A の合計と最大値を2行で出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let sum = a.iter().sum::<i64>();
    let max = *a.iter().max().unwrap();
    println!("{}", sum);
    println!("{}", max);
}
```

`sum::<i64>()` のターボフィッシュで型指定、`max` は `Option<&T>` を `unwrap()` + `*` でデリファレンス。

---

## 学んだこと

### `iter().sum()` の型指定

```rust
let sum = a.iter().sum::<i64>();   // ターボフィッシュ版
let sum: i64 = a.iter().sum();     // 型注釈版
```

- `sum` は **収集先の型を推論する必要がある** ので、どちらかの形で型を明示
- 「右辺で `::<T>()`」「左辺で `: T`」どちらも同じ
- ターボフィッシュは「右辺を読めば型がわかる」のでチェーンを書くときに便利

知識レベル: 🔵 説明可能

---

### `iter().max()` / `min()` は `Option<&T>` を返す

```rust
let max: i64 = *a.iter().max().unwrap();
```

- `max()` は空コレクションに対しては `None` を返す → `Option<&T>`
- 値を取り出すには `unwrap()` で `&T`、さらに `*` で `T`
- 「絶対空ではない」と保証できないと `unwrap()` でパニックの可能性。競プロは制約 $N \ge 1$ が多いので問題なし

知識レベル: 🟢 実装可能

---

### オーバーフローへの嗅覚

制約 $-10^9 \le A_i \le 10^9$、$N \le 100$ なら:
- 合計の最大は $N \times 10^9 = 10^{11}$
- `i32`（最大 $2 \times 10^9$）だと溢れる
- `i64` で受ければ安全

[[drill-a45]] の話と同じ。「合計を取る瞬間」にもう一度型サイズを意識する。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-a45]] — 型選びと $10^9$ ルール
- [[drill-a50]] — `windows` で隣接ペアを取る別パターン
