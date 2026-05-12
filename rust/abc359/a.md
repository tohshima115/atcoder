# A - Count Takahashi

Source: https://atcoder.jp/contests/abc359/tasks/abc359_a

配点 : $100$ 点

### 問題文

文字列が $N$ 個与えられます。

$i$ 番目 $(1\leq i\leq N)$ に与えられる文字列 $S _ i$ は `Takahashi` か `Aoki` のどちらかと等しいです。

$S _ i$ が `Takahashi` と等しい $i$ がいくつあるか求めてください。

### 制約

-   $1\leq N\leq 100$
-   $N$ は整数
-   $S _ i$ は `Takahashi` か `Aoki` のいずれか $(1\leq i\leq N)$

* * *

### 入力

入力は以下の形式で標準入力から与えられる。

$N$
$S _ 1$
$S _ 2$
$\vdots$
$S _ N$

### 出力

$S _ i$ が `Takahashi` と等しい $i$ の個数を整数として一行に出力せよ。

* * *

### 入力例 1

3
Aoki
Takahashi
Takahashi

### 出力例 1

2

$S _ 2,S _ 3$ の $2$ つが `Takahashi` と等しく、$S _ 1$ はそうではありません。

よって、`2` を出力してください。

* * *

### 入力例 2

2
Aoki
Aoki

### 出力例 2

0

`Takahashi` と等しい $S _ i$ が存在しないこともあります。

* * *

### 入力例 3

20
Aoki
Takahashi
Takahashi
Aoki
Aoki
Aoki
Aoki
Takahashi
Aoki
Aoki
Aoki
Takahashi
Takahashi
Aoki
Takahashi
Aoki
Aoki
Aoki
Aoki
Takahashi

### 出力例 3

7
