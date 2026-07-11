---
tags:
  - 全探索
  - ビットマスク
  - 集合演算
  - 座標リスト
  - グリッド
  - 回転
  - 図形
problem: abc322d
date: 2026-07-11
difficulty: abc-D
---

## 問題へのリンク

[ABC322 D - Polyomino](https://atcoder.jp/contests/abc322/tasks/abc322_d)

## 問題の要約

4×4 グリッドと、3 個のポリオミノ（`#` で表された連結図形）。**平行移動と回転（4通り）は自由・裏返し不可**で、3 個を重ならず・はみ出さず・全マス（16マス）を覆えるか判定する。

## 方針

- グリッドは 16 マスと小さい → **置き方を全列挙して総当たり**で間に合う規模。
- 判定を軽くするため、盤面を **16bit のビットマスク（`u16`）** で表す。
  - 重なり `a & b != 0` / 合体 `a | b` / 全マス埋まり `a | b | c == 0xFFFF`
- 図形の**回転・移動は「座標リスト」で持つと楽**。回転は座標変換 `(r,c) → (c,-r)` だけ。

**表現を2つ使い分けたのが肝**：図形を動かす処理は座標リスト、重なり判定はビットマスク。

## 実装アプローチ

図形を小さな部品関数に分けると一気に書ける。

```rust
// 4x4 の char グリッド → # の座標リスト（負が出るので i32）
fn to_cells(grid: &[Vec<char>]) -> Vec<(i32, i32)> {
    let mut cells = vec![];
    for j in 0..4 { for k in 0..4 {
        if grid[j][k] == '#' { cells.push((j as i32, k as i32)); }
    }}
    cells
}

// 90度回転：回転行列 (r,c)→(c,-r)。負が出ても後で詰めるので気にしない
fn rotate(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    cells.iter().map(|&(r, c)| (c, -r)).collect()
}

// 最小の行・列が 0 になるよう左上に詰める（正規化）
fn normalize(cells: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mr = cells.iter().map(|&(r, _)| r).min().unwrap();
    let mc = cells.iter().map(|&(_, c)| c).min().unwrap();
    cells.iter().map(|&(r, c)| (r - mr, c - mc)).collect()
}

// 正規化済みの形を、はみ出さない全平行移動でビットマスク化
fn placements(cells: &[(i32, i32)]) -> Vec<u16> {
    let mut res = vec![];
    let maxr = cells.iter().map(|&(r, _)| r).max().unwrap();
    let maxc = cells.iter().map(|&(_, c)| c).max().unwrap();
    for dr in 0..(4 - maxr) { for dc in 0..(4 - maxc) {
        let mut mask = 0u16;
        for &(r, c) in cells {
            let (nr, nc) = (r + dr, c + dc);
            mask |= 1 << (nr * 4 + nc);   // マス番号 = 行*4 + 列
        }
        res.push(mask);
    }}
    res
}

// 各ピースの「4回転 × 全配置」マスク一覧
fn all_masks(grid: &[Vec<char>]) -> Vec<u16> {
    let mut cells = to_cells(grid);
    let mut masks = vec![];
    for _ in 0..4 {
        cells = normalize(&rotate(&cells));
        masks.extend(placements(&cells));
    }
    masks
}
```

判定は3重ループ：

```rust
for &a in &m1 {
    for &b in &m2 {
        if a & b != 0 { continue; }              // 1と2が重なる
        for &c in &m3 {
            if (a | b) & c == 0 && (a | b | c) == 0xFFFF {
                // 発見 → Yes
            }
        }
    }
}
```

置き方は1ピース高々数十通り、3重でも数万〜数十万で余裕。

## このセッションで学んだこと

### 2次元グリッドは「操作」で表現を選ぶ

「グリッド = 座標リスト＆ビット」ではない。**やりたい操作**で選ぶ。

| 表現 | 得意なこと | 使う場面 |
|---|---|---|
| 2次元配列 `Vec<Vec<char>>` | 隣接マスを見る・書き換える | BFS/DFS・迷路・塗りつぶし（大多数はこれ） |
| 座標リスト `Vec<(i,j)>` | 回転・移動・点集合として扱う | 今回のような「図形を動かす」問題 |
| ビットマスク `u16` 等 | 重なり判定・合体・状態圧縮 | マスが少ない（〜20）時の集合演算 |

- 「隣を辿る」→ 配列
- 「形を動かす」→ 座標リスト
- 「重なり・集合演算・状態」→ ビット

詳しくは [[2d-grid-and-prefix]] / [[bit-operations]]。

知識レベル: 🟡 雰囲気理解（表現選択の判断は、もう数問こなして体に入れたい）

### 回転は座標変換 `(r,c) → (c,-r)` で済む

2次元配列を直接回そうとすると添字対応でバグる。**座標リストなら回転行列で一発**。負座標が出るが `normalize` で左上へ詰めれば形が揃う。座標を `usize` でなく `i32` にするのは、回転・移動で**一時的に負が出て `usize` だと panic する**ため（→ [[integer-types-overflow]]）。

知識レベル: 🟢 実装可能（対応表 `(r,c)→(c,-r)` と正規化のセットで書ける）

### ビットは「集合演算の翻訳」

`&`=積集合 / `|`=和集合 / `count_ones()`=要素数 / `(1<<n)-1`=全部入り。今回の `a & b != 0`（共通要素あり=重なり）、`a|b|c == 0xFFFF`（全マス）はこの翻訳で自然に読める。今回は厳密には bit全探索ではなく「マスク列挙＋総当たり」だが、**盤面をビットで持ち集合演算する発想**は同系統で、bitDP へも繋がる（bit全探索そのものは [[bit-operations]] 参照）。

知識レベル: 🟢 実装可能
