---
tags:
  - partition_point
  - 二分探索
  - 累積記録
  - hashmap
  - イベントソート
  - オフバイワン
  - 可変参照
  - get_mut
problem: abc464c
date: 2026-06-27
difficulty: abc-C
---

## 問題へのリンク

[ABC464 C - Plumage Palette](https://atcoder.jp/contests/abc464/tasks/abc464_c)

## 問題の要約

$N$ 羽の鳥は $D_i$ 日目を境に色 $A_i \to B_i$ に変わる。各日 $j=1\dots M$ について、その日の色の種類数を答える。$N, M \le 3\times10^5$。

## 方針（正しかった）

愚直に毎日 $N$ 羽を数えると $O(NM)$ で TLE。代わりに：

1. **色のカウントを HashMap で持つ**。`map.len()` がそのまま「色の種類数」になる。
2. 鳥を **$D$ の昇順にソート**して、$A\to B$ の切り替えを 1 羽ずつ処理。
3. 切り替えるたびに「現在の種類数」を配列 `bird_cnt` に記録していく（= 累積記録）。
4. 各日 $j$ について「$D_i \le j$ の鳥が何羽切り替わったか」を **`partition_point` で二分探索**し、その回数に対応する記録を引く。

計算量 $O(N \log N + M \log N)$。制約的に余裕。

## つまずいた点：オフバイワンが2段重なっていた

骨格・カウントロジックは完全に正しかったが、最後の「日 → 答え」の対応付けで2つのズレが重なり WA。サンプルでは `x=0` がたまたま正解と一致して気づきにくかった。

### ズレ① 日付のループ範囲

```rust
for x in 0..m {                              // x=0..m-1 を「日付」扱い → ズレる
    let i = ls.partition_point(|&l| l <= x);
```

$j$ 日目に切り替わっているのは $D_i \le j$ の鳥。日付 $j$ で回して `partition_point(|&l| l <= j)` を数えるべき。

### ズレ② 配列インデックス

`bird_cnt[k]` は「**$k+1$ 回**切り替えた後」を指していた（切り替え後に push しているため）。$k$ 回切り替え後がほしいなら `bird_cnt[k-1]`。

### きれいな直し方

ループ前に**初期状態（0回切り替え後）も push** しておくと、`bird_cnt[k]` がそのまま「$k$ 回切り替え後」になり、`-1` が消える。

```rust
// ループ前に初期状態を記録
bird_cnt.push(bird_list.len());
for &(a, d, b) in &bird {
    // ... aを減らしbを増やす ...
    bird_cnt.push(bird_list.len());
}
// クエリ：日 j に対して
for j in 1..=m {
    let k = ls.partition_point(|&l| l <= j); // D<=j の羽数
    println!("{}", bird_cnt[k]);             // k回切り替え後
}
```

そのほか、テンプレ由来の `println!("{}", n)` が残って **M+1 行出力**になっていた（不要）。

## このセッションで学んだこと

### `partition_point` で「条件を満たす個数」を数える

ソート済みスライスに対し、`v.partition_point(|&x| pred(x))` は **`pred` が `true` を返す要素の個数**（= false に切り替わる境界位置）を返す。`v` は `pred` について `true...true false...false` と分かれている前提。

```rust
let ls = vec![1, 3, 3, 3, 5, 6]; // D の昇順
ls.partition_point(|&l| l <= 3); // => 4 （D<=3 が4個）
```

「`x` 以下が何個あるか」を $O(\log N)$ で取れる。`lower_bound` 相当を自前実装せずに済む。

**知識レベル: 🟢 実装可能** — 「true群の個数を返す」と理解できれば使える。境界の不等号（`<=` か `<` か）でズレるので、サンプルで1点検算するクセをつける。

### HashMap のカウントで「種類数」を管理する

`map.len()` を種類数として使うには、**カウントが 0 になった色はキーごと削除**する必要がある。

```rust
// a を1減らす：1だったら消す、そうでなければデクリメント
if *map.get(&a).unwrap() == 1 {
    map.remove(&a);
} else {
    *map.entry(a).or_insert(0) -= 1;
}
*map.entry(b).or_insert(0) += 1; // b を1増やす
```

`A==B`（色が変わらない鳥）も、減らして同じ色を増やせば `len` が変わらず辻褄が合う。

**知識レベル: 🟢 実装可能**

#### もっとシンプルに：「減らしてから 0 判定」（後日レビュー）

上の「1 だったら remove」は、**先に減らしてから 0 になったか見る**ほうが素直。`a` は最初のループで必ず登録済みなので `entry().or_insert(0)`（無いかも、の書き方）ではなく `get_mut(&a).unwrap()` でいい。

```rust
let c = bird_list.get_mut(&a).unwrap(); // c: &mut i64（HashMap の中身への可変参照）
*c -= 1;
if *c == 0 {            // *c が c の最後の使用 → 借用が切れる
    bird_list.remove(&a); // のでここで remove してもエラーにならない（NLL）
}
*bird_list.entry(b).or_insert(0) += 1;
```

ポイントは順番。「減らす → 判定 → 消す」にすると、`if *c == 0` が `c` の最後の使用になり、その直後の `remove` で借用エラーにならない（Non-Lexical Lifetimes）。

**知識レベル: 🟢 実装可能**

### 可変参照（`&mut T`）は「コピー」ではなく「場所への矢印」

`get_mut(&a)` が返すのは値のコピーではなく **HashMap の中の値そのものへの可変参照 `&mut i64`**。だから `*c -= 1` が HashMap 側に波及する。

```rust
// ① get：&i64 を * でコピー → 独立した値。HashMap は変わらない
let c = *bird_list.get(&a).unwrap();
c -= 1;                              // HashMap に影響なし

// ② get_mut：&mut i64 → * で参照先（＝中身）を直接書き換え
let c = bird_list.get_mut(&a).unwrap();
*c -= 1;                            // HashMap の中身が変わる
```

`usize` の代入はコピーされる感覚に慣れていると引っかかるが、`&` / `&mut` は値のコピーではなく「その場所を指す矢印」。`*` で参照を辿ると元のデータに直接届く。元コードの `*map.entry(a).or_insert(0) -= 1` も同じ仕組み（`or_insert` が `&mut i64` を返す）。

**知識レベル: 🔵 説明可能** — 「参照は場所、`*` で実体を書き換える」と言葉で説明できるようになった。

### 同じ列を 2 本持たない：`partition_point` は記録配列に直接かける

提出版は二分探索用に `ls`（`d` だけの配列）を別に作っていたが、`bird_cnt` が既に `(種類数, d)` を持っているので不要。`bird_cnt` 自身に `partition_point` をかければ `ls` がまるごと消せる。

```rust
// let ls: Vec<usize> = ... ← 不要

for x in 1..=m {
    let i = bird_cnt.partition_point(|&(_, d)| d <= x);
    println!("{}", bird_cnt[i - 1].0);
}
```

`bird` を `d` でソート済み → `bird_cnt` も `d` 昇順 → そのまま二分探索できる。「同じ情報を 2 つの配列で持っていないか？」は冗長さを見つけるチェックポイント。

**知識レベル: 🟢 実装可能**

### 「イベントを時刻順に処理 → 累積記録 → 二分探索で引く」パターン

時間とともに状態が単調に変化し、各時刻の状態を多数問われる問題の定石：

1. 変化イベントを時刻順にソート
2. 1イベントずつ適用し、各時点のスナップショット（今回は種類数）を配列に貯める
3. 各クエリは二分探索でスナップショットを引く

毎クエリ再計算しないのがポイント。関連：[[../../topics/sort-cmp-ordering]]、[[../../reference/hashmap-hashset]]

**知識レベル: 🟡 雰囲気理解** — パターンとして認識できたが、初見で組み立てるにはもう数問慣れたい。次に類題を見たら手が動くか試す。
