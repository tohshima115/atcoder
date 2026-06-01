---
tags:
  - rust基礎
  - Vecメソッド
  - insert
  - 知らないと詰む
problem: drill-a38
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/a38|問題文]]

長さ N の数列 A の K 番目になる位置に値 X を挿入した数列（長さ N+1）を空白区切りで出力する。

## 実装アプローチ

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut a: [usize; n],
    }
    a.insert(k - 1, x);
    let s: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    println!("{}", s.join(" "));
}
```

最初は「前半・後半に分けて結合」というゴリ押し方針で書いたが、Rust の型システムにハマって完成しなかった。**Vec::insert を知っていれば1行**。

---

## 学んだこと

### `Vec::insert` — 指定位置に挿入

```rust
a.insert(pos, value);   // a[pos] に value が来て、それ以降は1個後ろにずれる
```

- K 番目（1-indexed）に挿入したいなら `a.insert(k - 1, x)`
- `pos == a.len()` なら末尾追加（`push` と同じ）になる
- `mut` 必須

知識レベル: 🟢 実装可能

---

### Vec の「追加・削除」メソッド早見表

| やりたいこと | メソッド | 戻り値 |
|---|---|---|
| 指定位置に挿入 | `a.insert(pos, x)` | `()` |
| 末尾に追加 | `a.push(x)` | `()` |
| 末尾を取り出す | `a.pop()` | `Option<T>` |
| 指定位置を削除 | `a.remove(pos)` | 削除した要素 |
| Vec を後ろに連結 | `a.extend(b)` / `a.append(&mut b)` | `()` |

A問題で出る数列の追加・削除はこの5つで全部回せる。

知識レベル: 🟡 雰囲気理解

---

### ゴリ押し方針が詰まる理由（Rust 特有）

最初に書いたコード:
```rust
let mut a_bef = a[..k].iter().collect();   // 型未定でエラー
let a_aft = a[k..].iter().collect();
a_bef.push(x);
let merge = a_bef + a_aft;                  // Vec の + は存在しない
```

ハマりポイント:

1. **`collect()` は収集先の型注釈が必須**: `Vec<&usize>` か `Vec<usize>` か決まらないとコンパイル不能
2. **`Vec` に `+` 演算子はない**: 連結したいなら `extend` か `[a, b].concat()`
3. **参照と値の型ミスマッチ**: `a.iter().collect::<Vec<_>>()` だと `Vec<&usize>` になり、`push(x: usize)` で型不一致

→ 「分けて結合」方針は Rust だと型変換が面倒。**標準メソッドを知っているか** が勝負。

知識レベル: 🔵 説明可能

---

### 「知ってるか知らないか」ゲーの戦略

A問題の数列操作はだいたい `Vec` の標準メソッドで1行になる。詰まったら:

- まず **やりたい動作の英単語**を考える（swap, insert, reverse, rotate, sort...）
- それを公式doc <https://doc.rust-lang.org/std/vec/struct.Vec.html> で Ctrl+F する
- 大体生えてる

ループで自力実装は最終手段。

知識レベル: 🟡 雰囲気理解

---

## 関連ノート

- [[drill-a36]] — `reverse`（区間反転）
- [[drill-a37]] — `swap`（要素交換）
- [[drill-a35]] — Vec 作成と join 出力の定型
