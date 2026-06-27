---
tags:
  - rust基礎
  - Vec
  - スライス
  - 標準ライブラリ
  - リファレンス
  - 計算量
problem: vec-slice-operations
date: 2026-06-27
difficulty: reference
---

# Vec / スライスの基本操作 と HashSet との違い

配列(`Vec`)の中身をいじる基本操作と、「あるか/ないか」を扱う `HashSet` との使い分けをまとめる。
`Vec` は **順番・添字が主役**、`HashSet` は **存在判定が主役**。得意分野が逆。

関連: [[hashmap-hashset]] / [[references-deref]]

---

## HashSet vs Vec — 同じ操作でも計算量が違う

`let mut v: Vec<i64> = vec![3, 1, 4];` に対して、HashSet の3操作を Vec でやると：

| 操作 | HashSet | Vec で同じことをやると | Vec の計算量 |
|---|---|---|---|
| **追加 insert** | `s.insert(x)` O(1) | `v.push(x)`（末尾追加）。重複を防ぐなら `if !v.contains(&x) { v.push(x) }` | push は O(1) / 重複チェック付きは O(N) |
| **存在確認 contains** | `s.contains(&x)` O(1) | `v.contains(&x)`（**先頭から全部見る**） | **O(N)** |
| **削除 remove** | `s.remove(&x)` O(1) | 位置を探して消す（下記） | **O(N)** |

**ポイント**: Vec の `contains`/`remove` は端から線形探索なので **O(N)**。
→ ループ N 回の中で `v.contains` を呼ぶと **全体 O(N²)** で TLE（N=2×10⁵）。
「あるか/ないか」を高速に判定したい場面は **HashSet を使う**。これが HashSet の存在理由。

**使い分けの軸**:
- 順番・添字が大事 → **Vec**
- 「あるか/ないか・重複排除」を高速に → **HashSet**
- 「キー → 値の対応」→ **HashMap**（[[hashmap-hashset]]）

知識レベル: 🟢 実装可能

---

## Vec の要素を変更する基本操作

### 1要素だけ

```rust
v.push(x);        // 末尾に追加         O(1)
v.pop();          // 末尾を取り出す      O(1)  → Option<T>
v[i] = x;         // 添字 i を書き換え    O(1)
let e = v[i];     // 添字 i を読む        O(1)  （範囲外はパニック）
let e = v.get(i); // 安全版               O(1)  → Option<&T>
```

### 全要素を条件で書き換え → `iter_mut()` ＋ `*e =`

詰まりやすい筆頭。**ただの `for e in &v` は読み取り専用**で書き換えられない。
書き換えたいときは **`iter_mut()`（可変イテレータ）** を使い、`*e` で参照の中身に代入する。

```rust
// v の中の 3 を全部 99 に変える
for e in v.iter_mut() {     // e: &mut i64（書き換え可能な参照）
    if *e == 3 {
        *e = 99;            // * で参照の中身に代入
    }
}
```

3種類の for の違い（[[references-deref]]）:

```rust
for e in &v           { /* e: &i64      読むだけ */ }
for e in v.iter_mut() { /* e: &mut i64  読み書きできる */ }
for e in v            { /* e: i64       所有権ごと（v はもう使えない） */ }
```

元を壊さず新しい Vec を作るなら `map`:

```rust
let w: Vec<i64> = v.iter().map(|&e| if e == 3 { 99 } else { e }).collect();
```

知識レベル: 🟢 実装可能

### 範囲をまとめて書き換え → スライス ＋ `.fill()`

「ここからここまでを全部この値に」はループ不要。**`.fill()` 一発**。

```rust
v[2..5].fill(7);     // 添字 2,3,4 を 7 に（右は含まない）
v[2..=5].fill(7);    // 添字 2,3,4,5 を 7 に（=5 で右も含む）
v.fill(0);           // 全部リセット
```

`v[2..5]` の部分が **スライス（配列の一部を指す窓）**。中ではループしてるが O(範囲長)、自分で書く必要なし。
⚠️ `v` が `mut` であること、範囲が長さを超える（`v[2..100]`）と実行時パニックに注意。

知識レベル: 🟢 実装可能

### 条件で残す / 消す → `retain`

```rust
v.retain(|&e| e != x);   // x を全部消す（x 以外を残す）
v.retain(|&e| e % 2 == 0); // 偶数だけ残す
```

`remove` を繰り返すより安全で速い。「条件 true の要素だけ残すフィルタ」。

### 値で消す / 探す

```rust
// 値 x の位置を探す → Option<usize>
if let Some(pos) = v.iter().position(|&e| e == x) {
    v.remove(pos);       // 最初に見つかった x を1個消す（後ろが詰まる O(N)）
}
v.remove(2);             // 添字 2 を消して返す（O(N)）
```

知識レベル: 🟡 雰囲気理解

---

## スライスでよく使う集計

`v[l..r]` はそのまま集計メソッドが使える（範囲指定の流儀は下表）。

```rust
let s: &[i64] = &v[2..5];          // 範囲を読み取り専用で借りる
v[2..5].iter().sum::<i64>();       // 範囲の合計
v[2..5].iter().max();              // 範囲の最大 → Option
v[2..5].sort();                    // 範囲だけソート（mut 必要）
dst[0..3].copy_from_slice(&src[0..3]); // 別配列の一部をコピー（長さ一致必須）
```

### 範囲指定 `..` の流儀（スライス共通）

| 書き方 | 範囲 |
|---|---|
| `v[2..5]` | 2,3,4（**右は含まない**） |
| `v[2..=5]` | 2,3,4,5（`=` で右も含む） |
| `v[..5]` | 0〜4（先頭から） |
| `v[3..]` | 3〜末尾まで |
| `v[..]` | 全部 |

---

## 早見表（やりたいこと → Vec）

| やりたいこと | コード | 計算量 |
|---|---|---|
| 末尾に足す | `v.push(x)` | O(1) |
| 末尾から消す | `v.pop()` | O(1) |
| 添字でアクセス | `v[i]` / 安全版 `v.get(i)` | O(1) |
| 添字を書き換え | `v[i] = x` | O(1) |
| **全要素を条件で書き換え** | `for e in v.iter_mut() { *e = ... }` | O(N) |
| **範囲をまとめて代入** | `v[l..r].fill(x)` | O(範囲長) |
| 条件に合うものだけ残す | `v.retain(\|&e\| 条件)` | O(N) |
| 含むか（遅い） | `v.contains(&x)` | O(N) |
| 値の位置を探す | `v.iter().position(\|&e\| e==x)` | O(N) |
| 添字で消す | `v.remove(i)` | O(N) |

---

## 関連ノート

- [[hashmap-hashset]] — 存在判定・重複排除は HashSet、キー→値は HashMap
- [[references-deref]] — `&` / `iter_mut` / `*` の参照の話
- [[iterator-catalog]] — `map`/`filter`/`position` などイテレータ操作
