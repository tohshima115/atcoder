---
tags:
  - topic
  - Option型
  - Result型
  - unwrap
  - map_or
type: topic
date: 2026-06-01
---

# Option / Result の扱い

> **繰り返し課題**: `max()` や `position()` の戻り値が `Option` で、そのまま使えずエラー。`unwrap` をどこに付けるかで迷う。
> 出てきた問題: [[drill-a21]] [[drill-a23]] [[drill-a24]] [[drill-a49]] [[drill-a52]] [[drill-a53]]

## 2つの「箱」

| 箱 | 中身 | いつ返る |
|---|---|---|
| `Option<T>` | `Some(値)` / `None` | 値があるか無いか（`max`, `position`, `pop`…） |
| `Result<T, E>` | `Ok(値)` / `Err(e)` | 成功か失敗か（`parse`, `from_str_radix`…） |

競プロでは制約上「失敗しない」場面が多く、どちらも `unwrap()` で開ければ大体OK。

## なぜ Option になるのか

- `iter().max()` は空コレクションに最大値が無いので `Option`（[[drill-a24]]）
- `iter().sum()` は空でも 0 を返せるので **Option にならない**（対比して覚える）
- エラー文に `Option<...> cannot be formatted` が出たら「`unwrap()` 付け忘れ」のサイン（[[drill-a24]]）

## unwrap はどこに付ける？ ← 頻出ミス

`unwrap` は **1個の `Option`/`Result`** のメソッド。イテレータ全体には効かない。

```rust
// ❌ map の結果(イテレータ)に unwrap
s.chars().map(|c| c.to_digit(10)).unwrap()
// ✅ クロージャの中で「1個ずつ」unwrap
s.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
```

→ [[drill-a53]]。「今その時点で扱っている型は何か」を確認するのが判断法。

## ハンドリング早見表（[[drill-a52]] が決定版）

```rust
let opt: Option<usize> = a.iter().position(|&v| v == x);

match opt {                       // 両方処理・型が違ってもOK
    Some(i) => println!("{}", i+1),
    None    => println!("-1"),
}
opt.map_or(-1, |i| i as i64 + 1)  // 1行で「Noneならデフォルト, Someなら変換」
opt.unwrap_or(0)                  // 値だけ・Noneはデフォルト
if let Some(i) = opt { ... }      // Some の時だけ処理
opt.unwrap()                      // 絶対Noneでないと確信できる時だけ
opt.filter_map(...)               // イテレータ内の Some だけ集める（下記）
```

| 用途 | おすすめ |
|---|---|
| 両方処理したい | `match` |
| 1行で値を出したい | `map_or` / `unwrap_or` |
| Some の時だけ | `if let` |
| 絶対 None じゃない | `unwrap`（最終手段） |

## filter_map — 「Some だけ拾う」

```rust
s.chars().filter_map(|c| c.to_digit(10)).sum::<u32>()  // 数字だけ拾って合計
```

`map(...).filter(Some).unwrap()` を1ステップにしたもの（[[drill-a53]]）。

## 関連

- [[references-deref]] — `max().unwrap()` の後に `*` が要る理由（`Option<&T>`）
- [[match-exhaustiveness]] — Option/Result を match で網羅すると `_ =>` 不要
- [[sum-collect-turbofish]] — `sum` は Option にならない対比
