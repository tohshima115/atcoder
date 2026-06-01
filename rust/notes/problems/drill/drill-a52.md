---
tags:
  - rust基礎
  - イテレータ
  - position
  - find
  - Option
  - match
  - 網羅性
problem: drill-a52
date: 2026-06-01
difficulty: drill
---

## 問題

[[../../contests/drill/a52|問題文]]

長さ N の数列 A の中で値 X が最初に出現する位置（1-indexed）を出力する。無ければ `-1`。

## 実装アプローチ

**match 版（読みやすい）**:
```rust
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }
    match a.iter().position(|&v| v == x) {
        Some(i) => println!("{}", i + 1),
        None    => println!("-1"),
    }
}
```

**map_or 版（1行）**:
```rust
let ans = a.iter().position(|&v| v == x).map_or(-1i64, |i| (i + 1) as i64);
println!("{}", ans);
```

**if let 版**:
```rust
let mut ans: i64 = -1;
if let Some(i) = a.iter().position(|&v| v == x) {
    ans = (i + 1) as i64;
}
println!("{}", ans);
```

最初は `a.iter().find(x)` と書いていた → `find` も `position` も **クロージャを取る**、値を直接渡せない。

---

## 学んだこと

### `find` と `position` の違い

| メソッド | 戻り値 | 用途 |
|---|---|---|
| `find(\|&&v\| 条件)` | `Option<&T>` | **値**そのものが欲しい |
| `position(\|&v\| 条件)` | `Option<usize>` | **インデックス**が欲しい |
| `contains(&x)` | `bool` | 含むか否かだけ |

「位置が欲しい」と「値が欲しい」を取り違えるのが典型ミス。**位置 → `position`** で反射的に出せるように。

知識レベル: 🔵 説明可能

---

### `find` / `position` はクロージャを取る

```rust
a.iter().find(x);              // ← エラー: x は値、クロージャじゃない
a.iter().find(|&&v| v == x);   // OK
a.iter().position(|&v| v == x);// OK
```

- `find` のクロージャの引数は `&&T`（iter() の `&T` にさらに filter/find 側が `&` を1枚足すので）→ `|&&v|`
- `position` のクロージャの引数は `&T` だけ → `|&v|`

なぜ違うかは `find` の実装が `filter` 似で参照を1枚多く持っているため。とりあえず **`find` は `&&`、`position` は `&`** で覚える。

知識レベル: 🟡 雰囲気理解

---

### `Option` のハンドリング3パターン

```rust
let opt: Option<usize> = ...;

// ① match — 読みやすい
match opt {
    Some(i) => println!("{}", i + 1),
    None    => println!("-1"),
}

// ② map_or(default, f) — 1行で書ける
let ans = opt.map_or(-1i64, |i| (i + 1) as i64);

// ③ if let — 片側だけ処理したい時
if let Some(i) = opt {
    println!("found at {}", i);
}
// None の時は何もしない（または事前にデフォルト値を置く）

// ④ unwrap_or(default) — 値だけ取り出して None は default に
let i = opt.unwrap_or(0);   // None なら 0、Some(v) なら v

// ⑤ unwrap — None でパニック（絶対 None じゃないと確信できる時だけ）
let i = opt.unwrap();
```

| 用途 | おすすめ |
|---|---|
| 両方とも処理したい（型違ってもOK） | `match` |
| 1行で値を取り出したい | `map_or` / `unwrap_or` |
| Some の時だけ処理 | `if let` |
| 絶対 None じゃない | `unwrap`（最終手段） |

知識レベル: 🔵 説明可能

---

### `match` の網羅性 — `_ =>` が要らない理由

```rust
match opt {
    Some(i) => ...,
    None    => ...,
}
// _ => は不要（Option は2バリアントだけなので全網羅完了）
```

- Rust の `match` は **網羅性をコンパイラがチェック**
- 全バリアント書けば `_ =>` は要らない
- 書くと「到達不能なアーム」として警告されることも

#### 網羅できるか / `_` が必要かの早見表

| 型 | バリアント数 | `_ =>` |
|---|---|---|
| `Option<T>` | 2 (Some/None) | 全部書けば不要 |
| `Result<T, E>` | 2 (Ok/Err) | 全部書けば不要 |
| `Ordering` | 3 (Greater/Less/Equal) | 全部書けば不要 |
| `bool` | 2 | 全部書けば不要 |
| 自作 enum | バリアント数 | 全部書けば不要 |
| 整数・String・&str | 無限 | 必須 |

**Rust の `match` が偉い理由**: あとで enum にバリアント追加した時、網羅 `match` は **コンパイルエラー** で教えてくれる。`if/else if` だと「漏れ」がコンパイルを通ってバグになる。enum と `match` の組み合わせは安全。

知識レベル: 🔵 説明可能

---

### 1-indexed への変換を忘れない

```rust
match a.iter().position(|&v| v == x) {
    Some(i) => println!("{}", i + 1),   // i は 0-indexed なので +1
    None    => println!("-1"),
}
```

`position` は 0-indexed の `usize` を返す。問題が 1-indexed で出力を求めているので **+1 必須**。これも [[drill-a36]] からの繰り返しテーマ。

知識レベル: 🟢 実装可能

---

## 関連ノート

- [[drill-a44]] — `match` で全網羅して `unreachable!()` を消す
- [[drill-a30]] — `|&&x|` の参照外し（`find` と同じパターン）
- [[drill-a41]] — `Option<&T>` の `unwrap` + `*` パターン
- [[drill-a36]] — 1-indexed → 0-indexed の繰り返しテーマ
