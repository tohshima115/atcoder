---
tags:
  - 二分探索
  - partition_point
  - suffix-max
  - 累積max
  - 座標が広すぎる
  - 計算量
problem: abc463c
date: 2026-06-20
difficulty: abc-C
---

## 問題へのリンク

[ABC463 C - Tallest at the Moment](https://atcoder.jp/contests/abc463/tasks/abc463_c)

## 問題の要約

N 人の高橋くん。i 番目は身長 Hᵢ で、L ᵢ 分後に退室する（L は昇順で与えられる）。
Q 個のクエリ T に対し、「T + ½ 分後に部屋にいる人の身長の最大値」を答える。

T + ½ 分後に i がいる ⇔ **T + 0.5 < Lᵢ ⇔ Lᵢ > T**（L,T は整数なので strict）。
つまりクエリ T の答えは **「Lᵢ > T を満たす人の中での Hᵢ の最大値」**。

## ハマったこと — 「時刻でインデックスする配列」は座標が広すぎて死ぬ

最初の方針は `highest[t] = 時刻 t の答え` を全時刻ぶん埋める配列だった。考え方（後ろから prefix を塗る／答えが T について単調非増加）は正しかったが、**致命的な壁**:

- 制約は `1 ≤ Lᵢ ≤ 10⁹`、`0 ≤ Tᵢ < L_N`。
- 配列サイズは L_N まで＝最大 **10⁹**。`vec![0i64; 10⁹]` ≈ **8GB** で MLE。
- クエリ値が小さくても配列サイズは L_N で決まるので逃げられない。

→ 「時刻（座標）が 10⁹ まで広いのに、時刻でインデックスする配列を作ろうとした」のが敗因。
**配列は人 i（サイズ N）でインデックスし、時刻 T は二分探索で i に変換する**のが正解。

## 方針 — suffix max + 二分探索（O((N+Q) log N)）

L が昇順 ⇒「Lᵢ > T の人」は配列の**後ろの連続区間 `[i, N-1]`**。
その区間の H の最大が欲しい ⇒ 区間の左端 i さえ分かれば、**i 以降の suffix max** がそのまま答え。

1. **suffix max** を前計算: `smax[i] = max(H[i..])`（後ろから1パス、O(N)）
2. クエリ T ごとに **Lᵢ > T となる最初の i を二分探索**（`partition_point`、O(log N)）
3. 答えは `smax[i]`

## 実装アプローチ（写経版）

```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        takahashi: [(i64, usize); n],  // (H_i, L_i)、L は昇順
        q: usize,
        t: [usize; q],
    }

    // ① L だけ取り出した配列（昇順のまま）。二分探索で使う
    let ls: Vec<usize> = takahashi.iter().map(|&(_h, l)| l).collect();

    // ② suffix max: smax[i] = max(H[i], ..., H[n-1])
    let mut smax = vec![0i64; n];
    smax[n - 1] = takahashi[n - 1].0;          // 末尾は自分自身
    for i in (0..n - 1).rev() {                // 後ろから前へ1パス
        smax[i] = takahashi[i].0.max(smax[i + 1]);
    }

    // ③ 各クエリ: l > t を満たす最初の i を二分探索 → smax[i] が答え
    for &ti in &t {
        let i = ls.partition_point(|&l| l <= ti);
        println!("{}", smax[i]);
    }
}
```

検算（例1）: takahashi=(31,4)(26,5)(3,5)(15,9)、ls=[4,5,5,9]。
smax=[31,26,15,15]。クエリ 3,4,5,6 → i=0,1,3,3 → 31,26,15,15。一致。

## このセッションで学んだこと

### 1. 「座標が広い」ときは座標でインデックスしない
値の範囲が 10⁹ 級なら、その値で配列を作った瞬間に MLE。
**「配列は要素数 N で持つ／広い座標は二分探索で N の世界に翻訳する」** が定石。
（座標圧縮の発想の入口でもある。）
**知識レベル: 🟢 実装可能** — 今回の失敗で「サイズ＝座標の最大」が危険信号だと身についた。

### 2. suffix max（後ろからの累積 max）
`smax[i] = max(H[i], smax[i+1])`。「i 以降の最大」＝「自分」と「i+1 以降の最大」の大きい方。
**末尾を先に決めて後ろから前へ1パス**で埋める。前から作ろうとすると未来の値が要るので作れない。
ループは `for i in (0..n-1).rev()`。prefix max（前から）の鏡像。

```rust
let mut smax = vec![0i64; n];
smax[n - 1] = h[n - 1];
for i in (0..n - 1).rev() {
    smax[i] = h[i].max(smax[i + 1]);
}
```
**知識レベル: 🟢 実装可能**

### 3. `partition_point` — オフバイワンしない二分探索
ソート済み配列で「条件 true の区間 / false の区間 の境目」のインデックスを返す。

```rust
let i = ls.partition_point(|&l| l <= ti);
// [ l<=ti が続く ... | l>ti が続く ... ]
//                     ↑ この境目 i が返る
```

- 述語は **前半 true → 後半 false の単調** になっている必要がある（ソート済みが前提）。
- `while lo<hi { mid=... }` を手書きするより境界ミスが出ない。**競プロ頻出の武器**。
- 「`l <= ti` の人（去ってる）を飛ばし、最初の `l > ti`（まだいる）の位置」を一発で取れる。

関連: lower_bound 相当は `partition_point(|&x| x < key)`、upper_bound 相当は `partition_point(|&x| x <= key)`。
**知識レベル: 🟡 雰囲気理解** — 使えたが、述語の向き（`<` か `<=` か）を毎回考えないと不安。手書き二分探索もいずれ書けるようにしたい。

### 4. 境界の存在保証は問題文の制約から読む
「T+½ 分後に必ず1人以上いる」保証があるので `l > ti` の人は必ず存在 → `i < n` が保証され、
`smax[i]` の範囲外チェックが不要。**制約を読むと if を1個減らせる**。
**知識レベル: 🔵 説明可能**

## 次にやること
- 手書き二分探索（`while lo < hi`）も一度書いて、`partition_point` の中身を腹落ちさせる。
- 座標圧縮（coordinate compression）の典型に進む。今回の「広い座標→ N の世界へ」の延長。
