---
tags:
  - 2次元グリッド
  - タプル
  - Vec
  - イテレータ
  - 参照
  - Copy
  - proconio
  - join
  - rust基礎
problem: drill-g1-g3
date: 2026-06-13
difficulty: drill
---

## 問題

[[2d-grid-drill]] の G1〜G3。2次元グリッドを「触る回数」を稼ぐドリル。

- **G1** セルの値を引く（1-index → 0-index 変換、クエリ処理）
- **G2** 各行の和・各列の和（二重ループの向き、1行出力）
- **G3** 時計回り90°回転（実行時サイズの2次元、String↔Vec<char>）

機構の辞書は [[2d-grid-and-prefix]]。

---

## G1 セルの値を引く

### proconio はタプルを直接読める

「`r c`」のクエリを `String` で読んで後から usize に変換…は**遠回り**。proconio はタプルを直接パースできる。

```rust
input! {
    h: usize, w: usize,
    grid: [[i64; w]; h],
    q: usize,
    query: [(usize, usize); q],   // "1 1" を (1,1) として直接読む
}
for &(r, c) in &query {
    println!("{}", grid[r - 1][c - 1]);   // 1始まり → -1 で 0始まりへ
}
```

🟢 実装可能（String→変換の二段構えを消せると気づけた）

### タプル型とは

「**意味の違う値を、決まった個数だけ束ねた箱**」。中身の型はバラバラでOK。

- アクセス: `t.0` `t.1`
- 分解（デストラクチャ）: `let (r, c) = t;` ← これが主役
- `for &(r, c) in &v` の `(r, c)` はこの分解

🔵 説明可能

### タプル vs 配列の使い分け

| | 配列 `[T; 2]` | タプル `(A, B)` |
|---|---|---|
| 中身の型 | 全部同じ | バラバラOK |
| アクセス | `a[0]` 添字 | `t.0` |
| 分解 | × | `let (r,c)=` ◎ |
| 全要素ループ | できる | できない（個数固定の別物） |
| 向く用途 | 同種の値の列 | 意味の違う数個の組（座標など） |

指針: **同じ意味の値が並ぶ → Vec／配列**、**意味の違う数個の組 → タプル**。
座標 `(r, c)` はタプルが自然。

🔵 説明可能

---

## G2 各行・各列の和

### 二重ループの「向き」

- 行の和: `g[i].iter().sum::<i64>()` を行ごとに `println!`（1個ずつ別行）
- 列の和: 外 `i` を列(0..w)、内 `j` を行(0..h) で回し `col[i] += g[j][i]`

> 罠: `g[j][w]` と書くと `w` は幅そのもので**常に範囲外**。列の添字は今の列変数 `i`。
> 変数名も `row` ではなく `col_sum` にしておくと `row`↔`col` の取り違えを防げる。

🟢 実装可能

### 「スペース区切り1行」の出力イディオム

`join` は**文字列スライスにしか効かない**ので、数値は一度文字列化が必要。

```rust
// 定番テンプレ
let ans: Vec<String> = col.iter().map(|x| x.to_string()).collect();
println!("{}", ans.join(" "));
```

```rust
// itertools があれば一発（to_string も collect も不要）
use itertools::Itertools;
println!("{}", col.iter().join(" "));
```

次から「スペース区切り1行」は `iter().join(" ")` を第一候補に。

🟡 雰囲気理解（itertools の join が効く条件はまだ曖昧）

---

## G3 時計回り90°回転

### 実行時サイズの2次元は Vec<Vec<_>>

`[[T; h]; w]` の固定長配列は **`h`/`w` が定数でないと作れない**。入力で読む値は実行時なので不可。
→ サイズが実行時に決まる2次元は `Vec<Vec<_>>`。

```rust
let mut ans = vec![vec![' '; h]; w];   // W行・各長さH、' ' で埋める
for i in 0..w {
    for j in 0..h {
        ans[i][j] = grid[h - 1 - j][i];   // new[i][j] = old[h-1-j][i]
    }
}
```

（`default::<char>` という構文は無い。デフォルトが要れば `char::default()` だが、`' '` 埋めで十分）

🟢 実装可能

### String ↔ Vec<char>

```rust
let grid: Vec<Vec<char>> = g.iter().map(|row| row.chars().collect()).collect(); // String→Vec<char>
let s: String = row.iter().collect();   // Vec<char>→String（出力時）
```

🟢 実装可能

### 参照のループと Copy

`for row in &ans`（`ans: Vec<Vec<char>>`）の1周は **1行 = `&Vec<char>`**。

- `&ans` … 借りるだけ。`row: &Vec<char>`、`ans` は残る
- `ans` … ムーブして消費、あとで使えない

**なぜ `for &row in &ans` と書けないか**: `&row` は「参照を剥がして中身を取り出す（ムーブ）」の意味。
中身 `Vec<char>` は **Copy でない**ので借用から持ち出せずエラー。

| 中身の型 | Copy? | `for &x in &v` |
|---|---|---|
| `i64` / `char` / 小さなタプル | ◯ | できる（コピーで取り出す） |
| `Vec<_>` / `String` | × | ダメ（参照のまま受ける） |

指針: **中身が数字・char・小タプル → `&` で剥がしてOK／中身が Vec・String → 剥がさず参照のまま**。
G1 の `for &(r,c) in &query` が剥がせたのは `(usize,usize)` が Copy だから。

🔵 説明可能

---

## このセッションの収穫

- **タプル**の正体（意味の違う値の組／分解）と配列との使い分けが言語化できた
- **実行時サイズの2次元 = Vec<Vec<_>>** が固まった（固定長配列との境界）
- **`&` で剥がせるか = 中身が Copy か** という一本の基準を獲得（G1〜G3 で一貫）

## 関連

- [[2d-grid-drill]] — ドリル本体
- [[2d-grid-and-prefix]] — 機構の辞書（①〜⑤）
- [[competitive-rust-index]] — 逆引きトップ
