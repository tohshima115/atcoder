---
tags:
  - rust基礎
  - 参照
  - 借用
  - clone
  - collect
  - イテレータ
  - sort
  - dedup
problem: drill-b2
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../../contests/drill/b2|問題文]]

$N$ 人が好きな食べ物を1つ回答。最も多く票を得た食べ物の得票数を出力する（人気投票・最大得票数）。

## 実装アプローチ

**提出コード（修正後）**:
```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut ans: i64 = 0;
    let mut list: Vec<String> = s.clone();
    list.sort();
    list.dedup();
    for v in &list {
        let cnt: usize = s.iter().filter(|&x| x == v).count();
        if cnt as i64 > ans {
            ans = cnt as i64;
        }
    }
    println!("{}", ans);
}
```

方針：`s` を複製して `sort` → `dedup` で候補（ユニークな食べ物）を作り、各候補について `s` 全体での出現回数を数えて最大を取る。制約 `N ≤ 100` なので O(N²) で間に合う。

> 計算量を詰めるなら `HashMap<String, usize>` で1パスで数えれば O(N)。今回は N が小さいので素直な方で十分。

---

## 学んだこと

### 詰まった原因はすべて「参照の枚数」だった

最初に書いたコードでハマった3箇所、全部 `&` まわり。

**① `*s.iter().collect()` — 型が合わない（コンパイルエラー E0282）**
```rust
// ❌
let mut list: Vec<String> = *s.iter().collect();
```
- `s.iter()` は `&String` を返す → `collect()` すると `Vec<&String>`（参照のVec）になる
- 左辺は `Vec<String>` を要求 → 型が合わない
- 先頭の `*` は Vec 全体をデリファレンスできないので無意味

```rust
// ✅ 丸ごと複製したいだけ
let mut list = s.clone();   // Vec<String> がそのまま手に入る
```
「`s` のコピーがほしい」だけなら `iter().collect()` を経由せず `.clone()` 一発。
（イテレータ経由なら `s.iter().cloned().collect::<Vec<String>>()`：各 `&String` を `cloned()` で `String` に複製してから集める）

知識レベル: 🟢 実装可能

**② `for &v in &list` — 参照の向こうから move できない**
```rust
for &v in &list { ... }   // ❌ &String から String を取り出そうとして move エラー
for v in &list { ... }    // ✅ v: &String のまま借りる
```
`&list` を回すと要素は `&String`。`&v` パターンで受けると「`v` に `String` を持ち出す」意味になるが、`String` は `Copy` でないので参照越しに move できない。

知識レベル: 🟢 実装可能

**③ `x == v` — 両辺の `&` の枚数を揃える**
```rust
s.iter().filter(|&x| x == v).count();
```
- `s.iter()` → `&String`、`filter` はさらに参照を付けて `&&String` を渡す
- `|&x|` で1枚剥がして `x: &String`
- `v` は `&String`（②で `for v in &list` にしたから）
- → `x == v` は `&String == &String` で枚数が揃う ✓

知識レベル: 🔵 説明可能

---

### `&` の枚数で詰まったときの機械的対処

エラーメッセージ `expected X, found Y` を見て：

| メッセージ | 対処 |
|---|---|
| `expected bool, found &bool` | `*` を1枚足す（剥がす） |
| `expected &String, found String` | `&` を1枚足す（包む） |

**足りない＝包む（`&`）、多い＝剥がす（`*` or パターンの `&`）**。運ではなく枚数合わせ。

参照の枚数の出発点になる原則：
- `vec.iter()` / `&vec` を回す → 要素は `&T`（参照）
- `vec.into_iter()` / `for x in vec` → 要素は `T`（値・所有権ごと）
- `str.chars()` → `char`（値・Copy）
- `filter`/`find` など「見るだけ系」はさらに `&` を1枚足してくる

知識レベル: 🟢 実装可能

---

### `sort` + `dedup` で重複除去

```rust
list.sort();
list.dedup();
```
`dedup()` は**隣り合う重複**しか消さないので、必ず `sort()` してから使う。これで「ユニークな要素一覧」が作れる。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-b1]] — `iter().filter(|&&x| ...)`（同じ参照の枚数の話）
- [[drill-a54]] — `chars().filter(|&ch| ...)`（こちらは値なので1枚）
- [[drill-a30]] — `filter` で `|&&x|`
