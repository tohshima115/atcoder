---
tags:
  - rust基礎
  - String
  - 文字列
  - 標準ライブラリ
  - リファレンス
problem: str-string-methods
date: 2026-07-11
difficulty: reference
---

# &str / String のよく使うメソッド（iter/chars を経由しない系）

競プロで文字列を触るとき、`chars().collect::<Vec<char>>()` で殴らなくても
`&str`/`String` のメソッド一発で済むことが多い。その一覧。

**見分けの勘どころ**:
- `split` / `chars` / `lines` みたいに「**複数個に分解する**」系だけイテレータを返す → `collect` が要る
- `trim` / `replace` / `contains` / `parse` みたいに「**1個の結果を返す**」系は一発で完結（collect 不要）

きっかけ: `12.340 → 12.34`, `0.000 → 0` の末尾ゼロ削り問題。
`x.trim_end_matches('0').trim_end_matches('.')` だけで解けた。

関連: [[vec-slice-operations]]

---

## メソッドチェーンの仕組み（なぜ collect なしで流れるのか）

`trim_end_matches` は**イテレータではなく `&str` を直接返す**。だから collect で固める工程がない。

```rust
x.trim_end_matches('0').trim_end_matches('.')
```
は分解すると：
```rust
let tmp = x.trim_end_matches('0');   // "0."
let ans = tmp.trim_end_matches('.'); // "0"
```
`.` でつなぐのは「**前の返り値に対して次のメソッドを呼ぶ**」を繰り返してるだけ。中間変数を省いてるだけで魔法ではない。

> 返り値は `String` でなく `&str`（元の一部を指す借用スライス）。`println!` で出すだけなら問題ないが、所有権が欲しい場面では `.to_string()` を付ける。

---

## 末尾・先頭を削る（trim 系）
| メソッド | 何をする | 例 |
|---|---|---|
| `trim()` | 前後の**空白**を削る | `" ab ".trim()` → `"ab"` |
| `trim_end()` / `trim_start()` | 後ろだけ / 前だけの空白 | 末尾改行 `\n` 除去に便利 |
| `trim_end_matches('0')` | 末尾から指定文字を削る | `"12.30".trim_end_matches('0')` → `"12.3"` |
| `trim_matches('x')` | 前後両方から | |

## 分割する（← ここだけ collect が要る）
| メソッド | 何をする | 例 |
|---|---|---|
| `split(' ')` | 区切りで分割 | `.split(' ').collect::<Vec<_>>()` |
| `split_whitespace()` | 連続空白でもうまく分割 | スペース数不定でも安全 |
| `splitn(2, ',')` | 最大 n 個に分割 | 最初の区切りだけで割りたいとき |
| `lines()` | 行ごとに分割 | 複数行入力 |

## 探す・含むか調べる
| メソッド | 返り値 | 例 |
|---|---|---|
| `contains("ab")` | `bool` | 部分文字列があるか |
| `starts_with("ab")` / `ends_with(".txt")` | `bool` | 前方/後方一致 |
| `find('x')` | `Option<usize>` | 最初に現れるバイト位置 |
| `rfind('x')` | `Option<usize>` | 最後に現れる位置 |

## 置換・変形
| メソッド | 何をする | 例 |
|---|---|---|
| `replace("a", "b")` | 全部置換 | `"aaa".replace("a","b")` → `"bbb"` |
| `replacen("a","b",1)` | n個だけ置換 | |
| `to_uppercase()` / `to_lowercase()` | 大小変換 | |
| `repeat(3)` | 繰り返し | `"ab".repeat(3)` → `"ababab"` |

## 長さ・中身
| メソッド | 返り値 | 注意 |
|---|---|---|
| `len()` | `usize` | **バイト数**（ASCII なら文字数と一致） |
| `is_empty()` | `bool` | |
| `chars().count()` | 文字数 | 非ASCIIで正確に数えたいとき |

## 数値へ変換（超頻出）
```rust
let n: i64 = s.parse().unwrap();  // "123" → 123（返り値の型でパース先が決まる）
let x: f64 = s.parse().unwrap();  // "1.5" → 1.5
```

## 添字アクセスしたいとき（唯一 Vec 化が要る場面）
`s[i]` は **禁止**（UTF-8 で1文字のバイト数が不定なため）。代わりに：
```rust
let v: Vec<char> = s.chars().collect();  // v[i] で1文字ずつ触れる
let b = s.as_bytes();                    // ASCIIなら b[i] は u8、こっちが速い
```
ASCII 前提の競プロなら `as_bytes()` が軽い。
