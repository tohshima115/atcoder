---
tags:
  - 2次元配列
  - 累積和
  - グリッド
  - Rust書き方
  - topics
date: 2026-06-09
---

# 2次元配列とグリッド・累積和の書き方（Rust）

> ロジックは分かるのに**書けない**ときの辞書。abc461 D で詰まった所。
> ここは「アルゴリズムの説明」ではなく **Rustの構文** を引くためのページ。

## ① 2次元配列を作る

```rust
let mut grid = vec![vec![0i64; w]; h];   // h行 × w列、全部0
//              └ 外側がh個 ┘ └内側がw個┘
grid[i][j] = 5;       // i行j列にアクセス（[行][列]の順）
```

- `vec![要素; 個数]` の入れ子。**外側の個数 = 行数 h**、内側 = 列数 w。
- 順番を逆（`vec![vec![0;h];w]`）にすると添字が壊れるので注意。

## ② 文字列グリッド `Vec<String>` → 数字の2次元配列（D で詰まった所）

```rust
// s: Vec<String>（"1001" などが h 行）を Vec<Vec<i64>> に
let grid: Vec<Vec<i64>> = s
    .iter()                                    // 各行 &String を順に
    .map(|row| row.bytes()                     // 1行を1バイトずつ
        .map(|b| (b - b'0') as i64)            // '1'→1。文字コードの引き算
        .collect())                            // → その行が Vec<i64>
    .collect();                                // → 全体が Vec<Vec<i64>>
```

- `b - b'0'` = 「文字 `'1'` のコード − 文字 `'0'` のコード = 数値 1」。`to_digit(10)` より競プロでは定番。
- `collect()` が**二段**なのは「行を作る」「行を集める」の2階層だから。
- 関連：[[string-char-conversion]]

## ③ 二重ループで自分で埋める（①の別解）

```rust
let mut grid = vec![vec![0i64; w]; h];
for i in 0..h {
    for j in 0..w {
        grid[i][j] = (s[i].as_bytes()[j] - b'0') as i64;  // i行のj文字目
    }
}
```

- `s[i].as_bytes()[j]` で i 行目の文字列の j 文字目をバイトで取得。

## ④ 2次元累積和

```rust
let mut pre = vec![vec![0i64; w + 1]; h + 1];   // ← サイズは +1（番兵）
for i in 0..h {
    for j in 0..w {
        pre[i + 1][j + 1] = pre[i][j + 1] + pre[i + 1][j] - pre[i][j] + grid[i][j];
    }
}
// 長方形 [r1,r2) × [c1,c2) の和（0始まり・半開区間）
let sum = pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1];
```

- サイズを `h+1, w+1` にして `pre[0][*]=pre[*][0]=0`（番兵）にすると、境界の場合分けが消える。
- 漸化式は「上 + 左 − 左上の重複 + 自分」。長方形和は「全体 − 上 − 左 + 引きすぎた左上」（包除）。

## ⑤ 縦方向だけの累積和（D 本体：行ペア固定で使う）

```rust
let mut colpre = vec![vec![0i64; w]; h + 1];
for i in 0..h {
    for j in 0..w {
        colpre[i + 1][j] = colpre[i][j] + grid[i][j];
    }
}
// r1..=r2 行・列 j のバンド合計 = colpre[r2 + 1][j] - colpre[r1][j]
```

- abc461 D は「行ペア (r1,r2) を固定 → これで col[j] を作り 1次元に潰す → 和=K を数える（[[drill-b15]] の seen so far / HashMap）」。
- 計算量：4重ループ = 6×10¹⁰ = TLE、行ペア+1次元 = 1.25×10⁸ = GO。→ [[complexity-estimation]]

## 練習問題

- 鉄則 **a08**（整数グリッド）… ①④だけに集中できる。文字列変換なし
- 鉄則 **a09**（2次元 imos）… ②③の応用
- **abc461 D** … ②（文字列→グリッド）＋⑤（行ペア＋1次元和=K）

## 構文を引く場所（アルゴリズムでなく書き方）

- std doc：`Vec` / `str`（`bytes` `chars` `as_bytes`）/ `Iterator`（`map` `collect`）
- cheats.rs / proconio の docs.rs（`input!` の `[String; h]` 書式）
- ⚠️ 鉄則・AtCoder 公式解説は C++/Python のアルゴリズム説明。**Rust構文の穴には効かない**

## 関連ノート

- [[competitive-rust-index]] — 逆引きトップ
- [[string-char-conversion]] — 文字↔数値
- [[drill-b15]] — 和=K の数え方（seen so far）
- [[complexity-estimation]] — 計算量の GO/NO-GO
