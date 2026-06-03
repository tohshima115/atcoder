---
tags:
  - イテレータ
  - rev
  - DoubleEndedIterator
  - collect
  - 参照剥がし
  - copied
  - chars
  - String
  - rust基礎
problem: drill-b5
date: 2026-06-03
difficulty: drill
---

## 問題

[[../../../contests/drill/b5|問題文]]

文字列 `S` を受け取り、**逆順にして出力**する。（`hello` → `olleh`）

## 実装アプローチ

**提出コード**:
```rust
use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut vec: Vec<char> = s.chars().collect();
    vec.reverse();
    let ans: String = vec.iter().collect();
    println!("{}", ans);
}
```

正しく動くが、**「イテレータ → Vec → イテレータ」と一往復**している（①`collect()` で Vec に固め、③`iter()` でまたイテレータに戻す）。反転のためだけに `mut` な Vec を経由しているのがモヤっとの正体。

**改善案（3 行 → 1 行）**:
```rust
let ans: String = s.chars().rev().collect();
```

- `s.chars()` … 文字を前から吐くイテレータ
- `.rev()` … **読み出す向きを逆にするだけ**（Vec を作らない／並べ替えもしない）
- `.collect()` … 逆順に出てくる文字を `String` に固める（固める先は左辺の型注釈で決まる）

---

## 学んだこと

### `.rev()` は「並べ替え」ではなく「読み出す向きの反転」

`Vec::reverse()` と `Iterator::rev()` は別物:

| やりたいこと | 書き方 | 元データ | `mut` |
|---|---|---|---|
| **物理的に並べ替える** | `v.reverse()` | 反転される（壊れる） | 要る |
| **逆順で読む／逆順の新しい物を作る** | `v.iter().rev()` | 無傷 | 不要 |

`.rev()` は実際の並べ替えをせず「後ろから出すモード」にするだけ。だから元データはいじらず、`mut` も要らない。

知識レベル: 🟢 実装可能

---

### `.rev()` が使える条件 = DoubleEndedIterator

`.rev()` は **前からも後ろからも取り出せるイテレータ（DoubleEndedIterator）** にしか生えない。

- `s.chars()` … 文字列という「並び」の上を走る → 後ろ向き OK
- `v.iter()` / `&[T]` … 並びの上を走る → 後ろ向き OK
- `HashSet` のイテレータなど「順序の概念がない」もの → `.rev()` は生えない

```rust
s.chars().rev().collect::<String>()   // 文字列の逆順
v.iter().rev().collect::<Vec<_>>()    // Vec の逆順（参照のまま）
```

同じ `.rev()` が `chars()` でも `iter()` でも効くのは、どちらも「並びを順に吐くイテレータ」だから。

知識レベル: 🔵 説明可能

---

### `iter()` か `into_iter()` か（Vec を逆順にするとき）

```rust
v.iter().rev()        // &i64 が後ろから出る（v は残る）
v.into_iter().rev()   // i64 が後ろから出る（v を消費・所有権ごと持っていく）
```

逆順の `Vec<i64>` がほしいだけなら `into_iter` のほうが素直（後述の `copied()` が不要）:
```rust
let r: Vec<i64> = v.into_iter().rev().collect();   // copied() 不要
```

選び方の軸 = **「元の `v` を後でまだ使う？」**。使うなら `iter()`（借りる）、使わないなら `into_iter()`（奪う）。b1〜b2 の「参照の枚数」の話と地続き。

知識レベル: 🟢 実装可能

---

### `chars().rev()` に `copied()` が要らない理由（最重要）

「何が出てくるか」が `chars()` と `iter()` で違う:

| | 出てくるもの | `collect` 前に剥がす？ |
|---|---|---|
| `s.chars()` | `char`（**値**） | 不要 |
| `v.iter()` | `&T`（**参照**） | `copied()` / `cloned()` で剥がす |
| `v.into_iter()` | `T`（値・所有権ごと） | 不要 |

```rust
s.chars().rev().collect::<String>()            // char → そのまま String にできる
v.iter().rev().copied().collect::<Vec<i64>>()  // &i64 → copied() で剥がす
```

**なぜ `chars()` は値、`iter()` は参照なのか**（構造上の必然）:

- `v.iter()` は「**既にメモリ上にある `Vec<i64>` を借りて見せる**」もの。元データが存在するので、それを指す参照 `&i64` を渡すのが自然。
- `s.chars()` は「**UTF-8 のバイト列をその場でデコードして `char` を組み立てる**」もの。`char` は走査中に新しく作られる一時的な値で、文字列の中に `char` がそのまま並んでいるわけではない（UTF-8 は 1 文字 1〜4 バイトの可変長）。指すべき元の `char` が存在しないので、値を返すしかない。

→ **「`copied()` が要るか」は「`iter()` を使ったか」で決まる**。`chars()` も `into_iter()` も値を吐くので不要、`iter()` だけが参照を吐くので必要。

知識レベル: 🔵 説明可能

---

### `collect()` の固める先は型注釈で決まる

```rust
let s: String     = chars.collect();   // String になる
let v: Vec<char>  = chars.collect();   // Vec<char> になる
```

`collect()` は「何に固めるか」を**左辺の型注釈（または `collect::<T>()` のターボフィッシュ）から読み取る**。同じイテレータでも受け皿の型で結果が変わる。

知識レベル: 🟢 実装可能

---

## このセッションの総括

| 観点 | 学び |
|---|---|
| 回り道の正体 | 「イテレータ→Vec→イテレータ」の一往復。`rev()` で一直線にできる |
| `reverse()` vs `rev()` | 物理並べ替え（mut 要）vs 読み向き反転（無傷・mut 不要） |
| `.rev()` の条件 | DoubleEndedIterator（並びがある）。HashSet などには生えない |
| `copied()` の要否 | `iter()`（参照）なら要る／`chars()`・`into_iter()`（値）なら不要 |

**学び**: 「動く正解」を、`collect` の往復を削って一行に。鍵は「イテレータは Vec に固めなくても変換・反転できる」「参照を吐くのは `iter()` だけ」という 2 点。

---

## 関連ノート

- [[drill-b4]] — `(0..w)` は最初からイテレータ／`v[i].iter()` が `&i64` を吐く話の続き
- [[drill-b2]] — `clone` と参照の枚数（`copied()`/`cloned()` の使い分け）
- [[drill-b1]] — `iter().filter` と参照剥がし（`iter()` が参照を吐く話の原点）
