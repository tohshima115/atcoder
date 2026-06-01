---
tags:
  - rust基礎
  - Vecメソッド
  - rotate
problem: drill-a51
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a51|問題文]]

長さ N の数列 A を左に K 個ローテートして空白区切りで出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i64; n],
    }
    a.rotate_left(k);
    let ans: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", ans.join(" "));
}
```

`rotate_left(k)` を知っていれば1行。これも [[drill-a38]] と同じ「知ってるか知らないか」ゲー。

---

## 学んだこと

### `Vec::rotate_left` / `rotate_right`

```rust
a.rotate_left(k);    // 左に k 個ローテート（先頭 k 個が末尾に移動）
a.rotate_right(k);   // 右に k 個ローテート（末尾 k 個が先頭に移動）
```

- `mut` 必須（その場で書き換える）
- `k >= a.len()` でも安全（自動的に `k % len` で計算される、はず）
- O(N) で動く

自力実装するなら:
```rust
let b: Vec<i64> = a[k..].iter().chain(a[..k].iter()).copied().collect();
```
`chain` は2つのイテレータを連結する。

知識レベル: 🟢 実装可能

---

### 「知ってるか知らないか」ゲー再確認

[[drill-a38]] の早見表に書いたが、Vec の標準メソッドを知っているかどうかで実装速度が全然変わる。

| やりたいこと | メソッド |
|---|---|
| 左/右ローテート | `rotate_left(k)` / `rotate_right(k)` |
| 2つのイテレータ連結 | `chain` |
| 入れ替え | `swap` |
| 挿入 | `insert` |
| 末尾追加 | `push` |
| 反転 | `reverse` |

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a38]] — Vec のメソッド早見表
- [[drill-a36]] — `reverse` の「その場で書き換え」パターン
