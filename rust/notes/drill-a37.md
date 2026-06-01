---
tags:
  - rust基礎
  - Vecメソッド
  - swap
  - 1-indexed
problem: drill-a37
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a37|問題文]]

長さ N の数列 A の P 番目と Q 番目の要素を交換して空白区切りで出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        mut a: [usize; n],
    }
    a.swap(p - 1, q - 1);
    let s: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" "));
}
```

**最初に書いたコード**: `a[p+1]` と書いていた → 1-indexed → 0-indexed の変換は **`-1`**。`+1` ではない。

---

## 学んだこと

### `Vec::swap` — 2要素を入れ替える

```rust
a.swap(i, j);   // a[i] と a[j] を交換（mut が必要）
```

自分で書くなら:
```rust
let tmp = a[i];
a[i] = a[j];
a[j] = tmp;
```
これが1行で済む。知らないと tmp 経由で書くハメになる典型例。

知識レベル: 🟢 実装可能

---

### 1-indexed の罠（再）

- 「P番目」「Q番目」は 1始まり
- 配列インデックスは 0始まり
- **常に -1** で変換する。`+1` は間違い

[[drill-a36]] の `l-1..r` と同じパターン。「番目」と書いてあったら反射的に `-1` する習慣をつける。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-a36]] — 1-indexed → 0-indexed の罠（区間反転）
- [[drill-a38]] — 同じく Vec メソッド系（insert）
