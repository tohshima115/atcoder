# AtCoder 精進記録

AtCoderの精進記録です。RustとTypeScriptで解いてます。

## 環境

- WSL2 (Ubuntu) on Windows
- Rust 1.95.0 (cargo-compete)
- TypeScript (Node.js 22 / tsx)
- VSCode + rust-analyzer

## ディレクトリ

- `rust/` — cargo-compete で管理。`rust/abcXXX/src/bin/*.rs`
- `ts/` — acc + oj で管理。`ts/abcXXX/*/main.ts`

## 運用方針

2025年以降、AtCoder の Cloudflare 認証導入とメモリ表記 (MB→MiB) 変更で、
acc/oj/cargo-compete の自動ログイン・自動提出は壊れている。

このリポジトリでは:

- **問題取得とローカルテストだけ** acc / oj / cargo-compete を使う
- **提出はブラウザで手動コピペ**
- ログインは cookie 直接貼り付けで通す (下の「ログイン更新」参照)

## 便利コマンド (`~/.bashrc` で定義済み)

| コマンド | 意味 |
|---|---|
| `cct a` | `cargo compete test a` — Rustのローカルテスト |
| `ccd` | `cargo compete download` — テストケース再取得 |
| `ojt` | `oj t -c "npx tsx main.ts"` — TSのローカルテスト |
| `apmd` | 問題文を Markdown で取得 (詳細は下記) |

### apmd の使い方

問題文を AtCoder からフェッチして Markdown 形式で保存。ブラウザを開かずにエディタで読める。

```sh
apmd                    # cwdから自動検出 (ts/abcXXX/<letter>/ で実行)
apmd a                  # rust/abcXXX/ で実行、問題aを取得 → ./a.md
apmd abc400 c           # 任意の場所から → ./c.md
apmd abc400 --all       # 全問一括 → ./{a,b,c,...}.md
apmd https://atcoder.jp/contests/abc400/tasks/abc400_a   # URL指定
```

数式は `$...$` でラップされているので、KaTeX/MathJax 対応のMDプレビューア
(VSCode の `Markdown+Math` 拡張など) で開くとちゃんと表示される。

## 過去問練習フロー

既に終了したコンテストで精進する場合。問題ディレクトリを作って、ローカルでテストを回しながら解く。

### Rust の場合

```sh
cd rust
cargo compete new abcXXX        # 問題ディレクトリ + テストケース取得 (a〜f/g)
cd abcXXX
apmd a                          # 問題文をフェッチ → ./a.md (任意)
$EDITOR src/bin/a.rs            # 解答を書く
cct a                           # ローカルでサンプル確認 (= cargo compete test a)
# 通ったらブラウザで src/bin/a.rs を貼って提出
```

`cct` は `~/.bashrc` で定義した `cargo compete test` のエイリアス。
プロジェクト内ならどこから実行してもOK (`src/bin/` の中からでも動く)。

既にあるディレクトリでテストケースだけ取り直したい場合は `ccd` (`cargo compete download`)。
`testcases/` は git 管理外なので、clone 直後やテストが見つからない時はこれを実行。

### TypeScript の場合

```sh
cd ts
acc new abcXXX                  # 問題ディレクトリ生成 (a〜f/g) + テストケース取得
cd abcXXX/a
apmd                            # 問題文をフェッチ → ./problem.md (任意)
$EDITOR main.ts                 # 解答を書く
ojt                             # ローカルでサンプル確認 (= oj t -c "npx tsx main.ts")
# 通ったらブラウザで main.ts を貼って提出
```

acc が test/ を自動取得しない場合は次で明示取得:

```sh
oj d https://atcoder.jp/contests/abcXXX/tasks/abcXXX_a
```

`ojt` は `~/.bashrc` で定義した `oj t -c "npx tsx main.ts"` のエイリアス。問題ディレクトリ (`a/`, `b/` ...) に居る状態で実行する。

## ABC 参加フロー

リアルタイムで ABC に参加する場合。タイマーが回っているので、開始直後にディレクトリを作ってあとは解くだけにしておく。

### 開始前 (〜21:00)

1. ターミナル2枚、エディタを起動して待機
2. AtCoder のコンテストページを開いておく (https://atcoder.jp/contests/abcXXX)
3. 必要なら cookie が生きているか確認: `acc session` / `oj login --check https://atcoder.jp/`

### 開始と同時 (21:00)

どちらか1つ、その日に使う言語で実行:

```sh
# Rust
cd rust && cargo compete new abcXXX && cd abcXXX
apmd abcXXX --all                    # 全問の問題文をMDで一括取得

# TypeScript
cd ts && acc new abcXXX && cd abcXXX
apmd abcXXX --all                    # 全問の問題文をMDで一括取得
```

これで a〜g までのディレクトリ、サンプル入出力、問題文MDが一気に揃う。

### 各問題を解く (A → B → C → ...)

Rust:
```sh
$EDITOR src/bin/a.rs
cct a                           # ローカルでサンプル確認
# 通ったらブラウザで提出
```

TypeScript:
```sh
cd a
$EDITOR main.ts
ojt                             # ローカルでサンプル確認
# 通ったらブラウザで提出
cd ..
```

### 提出時の注意

- AC/WA はブラウザ側で確認 (`oj` での提出結果取得は提出系コマンドに依存していて壊れている)
- TypeScript で提出する言語は **TypeScript 5.x (Node.js)** を選択

### 終了後 (22:40〜)

- 解けなかった問題は解説 PDF / 解説放送を見て復習
- 解けた問題も別解があれば追加で書く
- コミットしておく (`git add . && git commit -m "abcXXX"`)

## ログイン更新

REVEL_SESSION の有効期限は半年程度。ただし期限前でもサーバ側で無効化される
ことがある (特にブラウザでログインし直すと古いセッションが無効化される)。
`not login` や `cargo compete new` で Username/Password を要求されたら、
**3つのツール全部**の cookie を更新する (1つでも漏れるとそのツールだけ弾かれる):

1. ブラウザの DevTools → Application → Cookies → `https://atcoder.jp` → `REVEL_SESSION` の値をコピー
2. **acc**: `~/.config/atcoder-cli-nodejs/session.json` の `REVEL_SESSION=` の値を差し替え
3. **oj**: `~/.local/share/online-judge-tools/cookie.jar` の `REVEL_SESSION="..."` の `"..."` の中身を差し替え
4. **cargo-compete**: `~/.local/share/cargo-compete/cookies.jsonl` の `"raw_cookie":"REVEL_SESSION=..."` の値を差し替え (←忘れやすい。これが漏れると `cargo compete new` で Username/Password を要求される)
5. `acc session` と `oj login --check https://atcoder.jp/` で確認 (cargo-compete は次の `cargo compete new` で確認)

> 補足: 開催中の問題ページを無認証の `curl` で叩くと 404 が返る (参加者にしか
> 見えないため)。これは「未開始」の証拠にはならない。ブラウザで見えていれば取得可能。

## AtCoder Profile

[豊島](https://atcoder.jp/users/tohshima115)