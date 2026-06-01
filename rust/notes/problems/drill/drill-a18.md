---
tags:
  - rust基礎
  - match
  - 剰余
  - 型変換
problem: drill-a18
date: 2026-05-31
difficulty: drill
---

## 問題の要約

じゃんけん（rock / scissors / paper）の勝敗判定。A と B の手を受け取り、Takahashi / Aoki / Draw を出力する。

## 実装アプローチ

rock=0, scissors=1, paper=2 と番号を振り、剰余で勝敗を判定する。

```rust
fn jan_num(x: &str) -> i32 {
    match x {
        "rock" => 0,
        "scissors" => 1,
        "paper" => 2,
        _ => unreachable!(),
    }
}

let ans = if a_num == b_num {
    "Draw"
} else if (a_num - b_num + 3) % 3 == 2 {
    "Takahashi"
} else {
    "Aoki"
};
```

## このセッションで学んだこと

### 剰余を使った循環パターン

じゃんけんは「3つが輪になった勝敗関係」なので、剰余でエレガントに表現できる。

- rock=0, scissors=1, paper=2 と割り当てると
- A が B に勝つ ⟺ `(a - b + 3) % 3 == 2`
- +3 しているのは負の剰余を避けるため（`-1 % 3` は Rust では `-1` になる）

最初に `-1, 0, 1` で試したが、paper が rock に勝つ差が `1-(-1)=2` になり一定にならなかった。`0, 1, 2` にすることで全ての「勝ち」が差=2に統一される。

**知識レベル**: 🟢 実装可能

---

### `String` と `&str` の match

`String` 型の変数は文字列リテラル (`&str`) と直接 `match` できない。

```rust
// NG: String に対して &str パターンはコンパイルエラー
match x { "rock" => ... }

// OK: 参照を取る
fn jan_num(x: &str) -> i32 { match x { "rock" => ... } }
jan_num(&a)  // String から &str に変換

// OK: as_str() を使う
match x.as_str() { "rock" => ... }
```

**知識レベル**: 🟢 実装可能

---

### `unreachable!()` マクロ

`_` の catch-all に `unreachable!()` を使うと、「ここには絶対来ないはず」という意図を明示できる。

- 万が一来たらパニックして即気づける
- `_ => 1` のようにサイレントにデフォルト値を返すより安全

競技プログラミングでは `_` でも実用上問題ないが、`unreachable!()` の習慣を付けておくとバグを早期発見しやすい。

**知識レベル**: 🟢 実装可能

---

### Rust のシングルクォートはchar型

```rust
'a'   // char リテラル（1文字のみ）
"abc" // &str リテラル（文字列）
'abc' // コンパイルエラー（複数文字のchar不可）
```

**知識レベル**: 🔵 説明可能

---

### 関数の定義場所

Rust では `main` の内側に `fn` を書くことは合法だが、通常は外側に定義するのが慣習。

**知識レベル**: 🟡 雰囲気理解
