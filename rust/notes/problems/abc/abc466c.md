---
tags:
  - インタラクティブ
  - 尺取法
  - 標準入出力
  - 単調性
  - LineSource
problem: abc466c
date: 2026-07-11
difficulty: abc-C
---

## 問題へのリンク

[ABC466 C - Count Close Pairs](https://atcoder.jp/contests/abc466/tasks/abc466_c)

## 問題の要約

数直線上に点 $1,2,\ldots,N$ がこの順（左→右）で並ぶ。**座標は見えない**。`? i j` と質問すると「点 $i,j$ の距離が $1$ 以下か（Yes/No）」が返るインタラクティブ問題。**質問 $2N$ 回以内**で、距離 $1$ 以下の組 $(i,j)$ の個数を求める。$N \le 10^3$。

## 方針

- 素直に全ペア $\binom{N}{2} \approx N^2/2$ 回質問すると $N=10^3$ で約 $50$ 万回 → **$2N$ 制限に即アウト**。質問回数を線形に抑える工夫が要求されている。
- 点は左→右に並ぶ＝**座標が単調非減少**。よって $i$ を右にずらすと「$i$ から距離 $1$ 以内に収まる一番右の点」も**単調に右へしか動かない**。
  - → 右ポインタを $i$ をまたいで**持ち越す尺取法**が成立。これが今回の肝。
- 各 $i$ について右ポインタ `len` を「No が返るまで」進め、収まる個数 `len - i - 1` を足す。

**質問回数の見積もり**：`len` が進む回数は全体で高々 $N$ 回、各 $i$ で境界の No を取る回数が高々 $N$ 回 → 合計 **約 $2N$** で制約ぴったり。

## 実装アプローチ

提出コード（AC）：

```rust
use std::io::{stdin, stdout, BufReader, Write};
use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! { from &mut source, n: usize }

    let mut len: usize = 2;   // 右ポインタ。i をまたいで持ち越す
    let mut ans: usize = 0;
    for i in 1..n {
        let mut response = true;
        while response {
            if len <= n {
                if len == i { len += 1; }   // i == j の質問は出せないので飛ばす
                println!("? {} {}", i, len);
                stdout().flush().unwrap();
                input! { from &mut source, y: String }
                if y == "No" { response = false; }
                else { len += 1; }
            } else {
                response = false;
            }
        }
        ans += len - i - 1;
    }
    println!("! {}", ans);
}
```

### 改善案：`while + フラグ` を `loop + break` に

`response` を立て下げするより、`loop` で「抜ける条件のとき `break`」の方が意図が読みやすい。

```rust
loop {
    if len > n { break; }
    if len == i { len += 1; }
    println!("? {} {}", i, len);
    stdout().flush().unwrap();
    input! { from &mut source, y: String }
    if y == "No" { break; }
    len += 1;
}
```

`response` 変数が消え、「No か範囲外で抜ける」がそのまま読める。

## このセッションで学んだこと

### インタラクティブ問題の入出力テンプレ（初）

通常の `proconio::input!` は入力を一括バッファリングするので、**質問→応答を交互に繰り返す対話**では途中の入力が読めない。行単位で逐次読む `LineSource` を使う。

```rust
use std::io::{stdin, stdout, BufReader, Write};
use proconio::{input, source::line::LineSource};

let stdin = stdin();
let mut source = LineSource::new(BufReader::new(stdin.lock()));
input! { from &mut source, n: usize }   // from &mut source を毎回つける
```

- 出力のたびに **`stdout().flush().unwrap()`** が必須。しないと相手にデータが届かず TLE / 応答が来ずデッドロック。
- 応答は `input! { from &mut source, y: String }` で 1 トークン読み、`"Yes"`/`"No"` を文字列比較。
- 最後に `! X` を出して**ただちに終了**。

知識レベル: 🟡 雰囲気理解（テンプレとして貼れば動くが、`LineSource` の中身と flush の必要性は「そういうもの」レベル。次の対話問題でもう一度写して固めたい）

### 尺取法（two pointer）（初）

区間や右端を表すポインタを、**条件を満たす限り前へ進め、条件が崩れたら止める**。ポインタが後戻りしないので全体で線形回数。

**成立の前提は「単調性」**。今回なら「座標が単調 → $i$ が増えると到達可能な右端も単調に増える」。この単調性がないと右ポインタを持ち越せず尺取法にならない。→ ノートに残す価値のある一般則。

- `len` を各 $i$ でリセットしない（持ち越す）のが尺取法の本体。リセットすると $O(N^2)$ に戻る。
- カウント式 `len - i - 1`：`len` は「No が返った最初の点」を指すので、収まるのは $i+1 \ldots len-1$、個数は $(len-1)-(i+1)+1 = len - i - 1$。

知識レベル: 🟡 雰囲気理解（この問題では書けたが、「単調性を自分で見抜いて尺取法に落とす」判断は他の型（区間和・重複なし部分列など）で練習が要る）

### 境界の罠：`i == j` を避ける `len == i` スキップ

質問は $i < j$ が条件なので `? i i` は不正。`len` が持ち越しで `i` に追いついたとき（前の $i$ が即 No で終わり、次の $i$ が `len` と一致）だけ発生する。`if len == i { len += 1; }` で飛ばす。$i \le n-1$ なので `len += 1 \le n` に収まり、範囲外質問にはならない。**なぜこの1行が要るかを説明できることが大事**（抜けると WA）。

知識レベル: 🟢 実装可能（発生条件と対処をセットで説明できる）
