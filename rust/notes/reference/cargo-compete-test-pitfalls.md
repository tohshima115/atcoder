---
tags:
  - cargo-compete
  - cct
  - テスト
  - 落とし穴
  - 運用
date: 2026-06-13
---

# cct（cargo compete test）ローカルテストの落とし穴集

`cct` が WA を出しても、**コードが正しいのにテスタ側の都合で落ちている**ことがある。
本番 AtCoder ジャッジは寛容（末尾空白・改行を無視）だが、ローカルの比較は設定次第で厳しい。

## 鉄則：WA が出たら `note:` 行を必ず読む

cargo-compete は WA のとき、しばしば**直し方そのものを `note:` で教えてくれる**。

```
2/4 ("sample2") Wrong Answer (1 ms)
expected:
EMPTY
actual:

note:
whitespace-separated words matched. try setting `match` to `SplitWhitespace`
```

↑ この一行が答え。見落とすと20分溶ける（[[abc462a]] で実際にやらかした）。

---

## 落とし穴①：空出力サンプルが Lines 比較で偽WA

「空文字列を出力せよ」系のサンプル（例: [[abc462a]] の `codequeen`）で起きる。

- 自分の出力 = `println!` が末尾に `\n` を足す → **「空行が1つ」= 1行**
- 期待出力（`EMPTY`）= **「行が0個」**
- yml 先頭の `match: Lines` は**行単位の厳密比較**なので「1行 ≠ 0行」で WA

**コードは正しい。** `ans = ""` にしても `println!` が `\n` を足すので同じく落ちる。

### 直し方：テスタ側を直す（コードは触らない）

`testcases/<問題>.yml` の比較モードを変える:

```yaml
match: Lines            # ← これを
match: SplitWhitespace  # ← こうする
```

`SplitWhitespace` は「空白・改行を無視してトークン列で比較」。
空出力（単語0個）と空行（単語0個）が一致扱いになり、4/4 Accepted になる。

> `print!`（改行なし）にしても Lines を通せるが、「改行を出さない前提のコード」は
> 別の問題で事故るので**非推奨**。直すのは yml 側。

---

## 落とし穴②：SplitWhitespace は偽AC を生むことがある

`SplitWhitespace` は行構造を全部無視するので、

- ✅ 数値・単語の羅列が答え（ABC の大半）→ これでOK、本番ジャッジに近い
- ⚠️ **グリッドを複数行出力する系**など**行構造そのものが意味を持つ問題**
  → 行のズレを見逃して**偽AC**を出す危険。こういう問題のときだけ `Lines` に戻す

→ **デフォルト SplitWhitespace + 行が大事な問題だけ Lines** の使い分けが安全。

---

## 落とし穴③：設定は再ダウンロードで戻る

`match` は各問題の `*.yml` に書かれているので、

- 問題ごとに手で直す必要がある
- `ccd`（cargo compete download）で**再生成すると `Lines` に戻る**

毎回面倒なら `compete.toml` 側でデフォルト比較モードを変えられるはず（要調整。ただし②の偽ACリスク理解の上で）。

---

## チェックリスト（WA が出たとき）

1. `note:` 行を読む → 直し方が書いてあることが多い
2. `expected: EMPTY` で自分の出力が目視で正しい → 偽WA を疑い `match: SplitWhitespace`
3. 手で叩いて確認: `echo <入力> | cargo run --bin <contest>-<problem>`
4. それでも本当に違うならコードのバグ

## 関連

- [[abc462a]] — この落とし穴を踏んだ問題
- [[complexity-estimation]] · [[hashmap-hashset]]
