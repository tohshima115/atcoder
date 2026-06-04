---
tags:
  - HashSet
  - two-sum
  - seen-so-far
  - オンライン処理
  - 計算量
  - 鳩ノ巣
  - rust基礎
problem: drill-b15
date: 2026-06-04
difficulty: drill
---

## 問題

[[../../../contests/drill/b15|問題文]]

$N$ 個の整数から $i<j$ で $a_i + a_j = K$ となる組が**存在するか**判定（`Yes`/`No`）。$N \le 2\times10^5$。

## 実装アプローチ

[[drill-b6]] と同じ問題だが制約が $N \le 2\times10^5$。二重ループ $O(N^2)=4\times10^{10}$ は **TLE**（→ [[../../reference/complexity-estimation]]）。
**「これまでに見た数」の HashSet** を使い、各 $a_j$ で「相方 $K-a_j$ が既出か」を O(1) で問い合わせて $O(N)$。

```rust
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! { n: usize, k: i64, a: [i64; n] }
    let mut seen: HashSet<i64> = HashSet::new();
    let mut found = false;
    for &x in &a {
        if seen.contains(&(k - x)) {   // ① 相方が「これまでに」出たか
            found = true;
        }
        seen.insert(x);                // ② 調べた“後で”自分を入れる
    }
    println!("{}", if found { "Yes" } else { "No" });
}
```

---

## 学んだこと

### 「調べてから入れる」順序トリック（最重要）

危険な方針：**全部 set に入れてから** $K-x$ を探す → `x == K-x`（相方が自分と同じ値）のとき
**自分自身とペアを組んで誤答**する。例：`K=4, a=[2]`（2 が 1 個）で「2 が set にある」→ Yes と誤答（本当は組めない）。

正しい方針：set を**ループしながら育てる**。各 $x$ で「相方が**今まで**に出たか」を調べ、**その後で**自分を入れる。

```rust
if seen.contains(&(k - x)) { found = true; }  // 先に調べる
seen.insert(x);                                // 後で入れる
```

**順番が命**（①調べる → ②入れる）。これで「自分より前の要素」としか組めず、`i<j`（同じ要素を2回使わない）が**自動で守られる**。

`K=4, a=[2,2]` のトレース:
| 見る x | 調べる時の seen | $4-x$ が seen に? | 結果 | 追加後 |
|---|---|---|---|---|
| 1個目 2 | `{}` | 無 | × | `{2}` |
| 2個目 2 | `{2}` | **有** | **Yes** | `{2}` |

→ `a=[2,2]` は Yes、`a=[2]` は No。両方正しい。

知識レベル: 🔵 説明可能

### "seen so far" パターンは超汎用

「これまでに見たものの set/map」は two-sum 以外にも頻出:

- 「同じ値が**前に**出たか」→ 重複検出
- 「$x-K$ が前に出たか」→ 差が $K$ のペア
- 「累積和が前に出たか」→ 和が $0$（や $K$）の区間（少し先の典型）

共通の型：**「今の自分」が参照していいのは「自分より前」だけ → 先に調べ、後で登録**。
「入れる→調べる」にすると自分とペアを組んで事故る。この一行の順序が AC と WA を分ける。

知識レベル: 🟡 雰囲気理解

### これが「Map/Set が逃げられない」典型

「引きながら問い合わせる」**オンライン処理**なので sort では書けない。
[[drill-b6]]（$N$ 小 → $O(N^2)$ でOK）と同型でも、制約が上がると **HashSet で O(N)** が必須に。
→ Map/Set の逃げ道の限界（[[../../reference/map-when-and-btreemap]]）の実例。

知識レベル: 🟢 実装可能

### 細かい点

- `seen.contains(&(k - x))` … `contains` は**参照**を取るので `&(...)`。
- `k`/`x` は `i64`。`k - x` は負にもなり得るが i64 なら平気。
- 見つけ次第 `break` してもよい（最初の 1 組で確定）。

知識レベル: 🟢 実装可能

---

## このセッションの総括

| 学び | 要点 |
|---|---|
| 順序トリック | 「調べる→入れる」で `i<j` が自動で守られ、自己ペア事故を防ぐ |
| seen so far | 「これまで見た集合」は two-sum/重複/区間和で再利用できる型 |
| 逃げられない | 「引きながら問い合わせ」= sort 不可、HashSet で O(N) |

**学び**: $O(N^2)$ を $O(N)$ に落とす定番。鍵は「全部入れてから探す」ではなく「**これまで見た分だけの集合に、調べてから入れる**」。順序ひとつで正しさが決まる。

---

## 関連ノート

- [[drill-b6]] — 同じ two-sum の $O(N^2)$ 版（制約が小さい）
- [[../../reference/complexity-estimation]] — $N\le2\times10^5$ で $O(N^2)$ が TLE
- [[../../reference/map-when-and-btreemap]] — オンライン処理は Map/Set が必須
- [[../../reference/hashmap-hashset]] — HashSet の `contains`/`insert`
