---
tags:
  - 累積和
  - HashMap
  - seen-so-far
  - 区間和
  - 計算量
  - 紙とペン
  - rust基礎
problem: drill-b19
date: 2026-06-04
difficulty: drill
---

## 問題

[[../../../contests/drill/b19|問題文]]

$N$ 個の整数（負あり）で、連続部分列の和がちょうど $K$ になるものの個数を求める。$N \le 2\times10^5$。

## 実装アプローチ

二重ループ $O(N^2)$ は TLE。**累積和 + HashMap（回数）** で $O(N)$。

```rust
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! { n: usize, k: i64, a: [i64; n] }
    // 累積和を O(N) で作る（配列は一度作れば使い回す）
    let mut s = vec![0i64; n + 1];
    for i in 0..n { s[i + 1] = s[i] + a[i]; }   // 前の合計 + 今の1個

    let mut count: HashMap<i64, i64> = HashMap::new();
    let mut ans: i64 = 0;
    for j in 0..=n {                              // S_0, S_1, ..., S_n を順に
        let now = s[j];
        ans += *count.get(&(now - k)).unwrap_or(&0);  // now-k が今まで何回出たか
        *count.entry(now).or_default() += 1;           // now を登録
    }
    println!("{}", ans);
}
```

> ⚠️ やりがちな罠：累積和を `a[0..i].iter().sum()` で作ると**それ自体が O(N²)**。
> 必ず `s[i+1] = s[i] + a[i]`（前の値に1個足す）で O(N)。`reverse` は不要。

---

## 学んだこと

### 区間和 = K は「累積和の差」（和ではない）

区間 $(l, r]$ の和 $= S_r - S_l$。これが $K$ ⇔ **$S_r - S_l = K$（差）**。
「今いる累積和 $S_{now}$」に対し、**探すべき過去の値**を求めて式を解く:

$$S_{now} - S_{old} = K \;\Rightarrow\; S_{old} = S_{now} - K$$

→ `now - k` を探す。これは**差**を解いた形であって、和ではない。

**[[drill-b15]]（和）との対比**：look up する値が違う。

| | 探したい関係 | 式変形 | look up |
|---|---|---|---|
| b15 two-sum | $x + y = K$（**和**） | $y = K - x$ | `K - x` |
| b19 区間和 | $S_{now} - S_{old} = K$（**差**） | $S_{old} = S_{now} - K$ | `s - K` |

**和か差か → 相方について式を解く → look up する値が決まる**。累積和系で毎回効く考え方。

知識レベル: 🔵 説明可能

### HashSet では漏れる → HashMap（回数）が必要

累積和は**同じ値が複数回出る**（例 `[1,-1,1,-1,1]` の prefix は `0,1,0,1,0,1`）。
和が 0 の区間 = 同じ累積和のペア。`0` が 3 回出るなら ₃C₂ = 3 個の区間がある。

`HashSet`（存在のみ）だと「`0` は出た」しか分からず**回数を取りこぼす**→ 過少カウント。
`HashMap<累積和, 回数>` にして「何回出たか」を足す必要がある。

| | 知りたいこと | 道具 |
|---|---|---|
| 存在するか（Yes/No） | b15 | HashSet |
| **何回**あるか（個数を足す） | b19 | **HashMap** |

→ 「同じ数字が出るから Set じゃ漏れる」と気づけたら、それが Set→Map に上げる理由。

知識レベル: 🔵 説明可能

### `get`（読む）と `entry`（書く）の使い分け

```rust
ans += *count.get(&(now - k)).unwrap_or(&0);  // 読むだけ → get + unwrap_or
*count.entry(now).or_default() += 1;           // 書き換え → entry + or_default
```

`values()`（全部の値の列）ではなく `get(&キー)`（特定キーの回数）を使う。
「`now-k` という1つの累積和が何回か」が欲しいので `get`。

知識レベル: 🟢 実装可能

### `{0: 1}` の初期化 = ループを `j=0` から回す

「先頭から始まる区間」（$S_{old}=0$ を左端にする）を数えるには、空 prefix $S_0=0$ を「1回出た」として置く必要がある。
`for j in 0..=n` と **`a_sum[0]=0` を最初に処理**すれば、別途 `insert(0,1)` 不要（j=0 で 0 が登録される）。

知識レベル: 🟢 実装可能

### C 以降は「紙とペン」で式を立ててから書く

頭の中だけで `S_now - S_old = K` を移項…は誰でもきつい。**紙に書いてから実装**が正攻法（上位勢ほどやる）。
紙に書くと効くもの：小さい例の手計算 / 変数の意味（`S_i = 先頭i個の和`）/ 式変形（移項）/ DP の漸化式 / 0-indexed と 1-indexed の対応。

ただし定番（「区間和=K → 累積和+map」）は数回やると**見た瞬間わかるパターン**になる。
紙が要るのは「初めての形・ひねり」。B は反射、C は紙で詰める、が目安。

知識レベル: 🔵 説明可能（習慣として）

---

## このセッションの総括

| 学び | 要点 |
|---|---|
| 区間和=K | 累積和の**差** $S_{now}-S_{old}=K$ → `now-k` を探す |
| Set vs Map | 同じ累積和が複数回 → 回数が要る → HashMap |
| 累積和の構築 | `s[i+1]=s[i]+a[i]` で O(N)（`sum()` 再計算は O(N²) 罠） |
| 式の解き方 | 和か差か → 相方について解く → look up 値が決まる |
| 紙とペン | C 以降は紙で式を立ててから実装が正攻法 |

**学び**: [[drill-b15]] の seen-so-far を累積和に応用した C 頻出形。鍵は「和でなく差」「存在でなく回数」。そして難しくなったら紙に移すのが正しい。

---

## 関連ノート

- [[drill-b15]] — seen-so-far の原型（和・存在判定の HashSet 版）
- [[drill-b9]] — 累積和の基本（`s[i+1]=s[i]+a[i]`）
- [[../../reference/hashmap-hashset]] — `get`/`entry` の使い分け
- [[../../reference/complexity-estimation]] — O(N²) が TLE になる規模
