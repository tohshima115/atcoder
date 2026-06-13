---
tags:
  - 2次元グリッド
  - 境界処理
  - 番兵
  - クロージャ
  - dx-dy
  - 型変換
  - join
  - rust基礎
problem: drill-g4
date: 2026-06-13
difficulty: drill
---

## 問題

[[2d-grid-drill]] の G4。H×W 整数グリッドで、各セルの**上下左右4近傍の和**（存在するものだけ）を同じ形で出力。狙いは **境界処理（`i>0` `i+1<h` …）** の手癖づけ。

機構の辞書は [[2d-grid-and-prefix]]。

---

## 提出したコード（パディング方式）

周囲を0で囲った `(h+2)×(w+2)` の `frame` を作り、境界分岐を消す方針。

```rust
let mut frame = vec![vec![0i64; w+2]; h+2];
for i in 0..h {
    for j in 0..w {
        frame[i+1][j+1] = g[i][j];   // 中身を1マスずらして詰める
    }
}
for i in 0..h {
    let mut ans = vec![0i64; w];
    for j in 0..w {
        // (i,j) は frame では (i+1,j+1)。その上下左右
        ans[j] = frame[i][j+1] + frame[i+1][j] + frame[i+1][j+2] + frame[i+2][j+1];
    }
    println!("{}", ans.iter().join(" "));   // ← ここでコンパイルエラー
}
```

**ロジックとインデックスは正しかった**（番兵で境界分岐を消すのは正攻法）。
詰まったのは出力の1行だけ。

---

## このセッションで学んだこと

### `.join()` はイテレータには生えていない

`ans.iter().join(" ")` がコンパイルエラー（`no method named join found for ... Iter`）。

- イテレータの `.join(" ")` は **itertools の `Itertools` トレイト**のメソッド。`use itertools::Itertools;` が無いと存在しない。
- コンパイラは「`join` is available on `&[i64]`」と出すが、これは罠。**スライスの `join` は文字列やスライスの連結用**で、`i64` の Vec には使えない。

```rust
// 標準だけ: 一度 String 化してから繋ぐ（定番テンプレ）
println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));

// itertools があれば一発
use itertools::Itertools;
println!("{}", ans.iter().join(" "));
```

[[drill-g1-g3]] の G2 でも同じ罠を踏んでいる。「スペース区切り1行」は **`map(to_string).collect::<Vec<_>>().join(" ")`** を反射で出せるようにする。

🟢 実装可能（なぜ生えてないかまで言える）

### 境界処理の3つの書き方と使い分け

「周囲を0扱いしたいだけ」に対して、パディングのコピー二重ループは確かに重い。同じ O(HW) で書き方は3通り。

#### 案A: 境界外を0で返すクロージャ（この問題に一番素直）

枠もコピーも作らず、「範囲外を読んだら0」の関数を1個用意するだけ。境界判定が1箇所に集約される。

```rust
let get = |i: i64, j: i64| -> i64 {
    if 0 <= i && i < h as i64 && 0 <= j && j < w as i64 {
        g[i as usize][j as usize]
    } else {
        0
    }
};
// 各セル: get(i-1,j) + get(i+1,j) + get(i,j-1) + get(i,j+1)
```

#### 案B: dx/dy 配列 + 境界チェック（手癖にすべき本命）

ドリルの狙いそのもの。そして **P3 の BFS が全く同じ形**で書ける（近傍が8方向・斜めになっても `dij` を変えるだけ）。投資対効果が最大。

```rust
let dij: [(i64, i64); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
let sum: i64 = dij.iter().filter_map(|&(di, dj)| {
    let (ni, nj) = (i + di, j + dj);
    (0 <= ni && ni < h as i64 && 0 <= nj && nj < w as i64)
        .then(|| g[ni as usize][nj as usize])
}).sum();
```

`bool::then(|| ...)` … 条件が true なら `Some(値)`、false なら `None`。`filter_map` と組むと「範囲内のものだけ拾う」が書ける。

#### 比較表

| 方式 | 余分な配列 | コピーループ | 汎用性 |
|---|---|---|---|
| パディング（提出版） | 要る | 要る | 境界分岐は消える◎ |
| 案A getクロージャ | 不要 | 不要 | この手の問題に最適 |
| 案B dx/dy | 不要 | 不要 | **BFS等に直結◎** |

計算量はどれも O(HW) で同じ。**正しさでは差がない → 学習効果で選ぶなら案B**、この問題単体の読みやすさなら案A。

**パディングが本当に効く場面**：全マスで何度も近傍アクセスする／分岐を1つも書きたくない（畳み込み等）。そのときはコピーループのコストを払う価値が出る。今回は各マス1回しか見ないので案A/Bが素直。

🟡 雰囲気理解（dx/dy は書けるが、BFS への展開はP3で要確認）

### 添字計算で `i64` を経由する理由

近傍は `i-1` を作るので、`usize` のままだと `0 - 1` がアンダーフロー（panic）。
→ **添字計算は一旦 `i64` に上げて**、範囲チェックを通った後 `as usize` で戻す。競プロ頻出の型さばき。

```rust
for i in 0..h as i64 { ... let (ni, nj) = (i + di, j + dj); ... g[ni as usize][nj as usize] }
```

🟢 実装可能

---

## このセッションの収穫

- **`.join()` がイテレータに無い理由**（itertools 由来 / スライス join は連結用）を言語化。G2 と同じ罠なので次は反射で回避する。
- **境界処理は3通り**（パディング / getクロージャ / dx-dy）あり、計算量同じなら**汎用性で dx/dy を本命に**、という判断軸を得た。
- パディングは悪くない正攻法。重く感じたのは「枠を埋めるコピーループ」で、それは案A/Bで消せると分かった。
- **添字計算は i64 経由 → as usize で戻す**の型さばきを再確認。

## 関連

- [[drill-g1-g3]] — 同じ join 罠・2次元の基礎
- [[2d-grid-drill]] — ドリル本体（次は P3 BFS で dx/dy を実戦投入）
- [[2d-grid-and-prefix]] — 機構の辞書（①〜⑤）
- [[integer-types-overflow]] — usize アンダーフローと型さばき
- [[competitive-rust-index]] — 逆引きトップ
