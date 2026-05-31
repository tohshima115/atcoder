# E - Mod Sigma Problem

Source: https://atcoder.jp/contests/abc378/tasks/abc378_e

配点 : $475$ 点

### 問題文

長さ $N$ の非負整数列 $A=(A_1,A_2,\dots,A_N)$ と、正整数 $M$ が与えられます。

次の値を求めてください。

\$ \sum\_{1 \leq l \leq r \leq N} \left( \left(\sum\_{l \leq i \leq r} A\_i\right) \mathbin{\mathrm{mod}} M \right) \$

ここで、$X \mathbin{\mathrm{mod}} M$ で、非負整数 $X$ を $M$ で割ったあまりを表します。

### 制約

-   $1 \leq N \leq 2 \times 10^5$
-   $1 \leq M \leq 2 \times 10^5$
-   $0 \leq A_i \leq 10^9$

* * *

### 入力

入力は以下の形式で標準入力から与えられる。

$N$ $M$
$A_1$ $A_2$ $\dots$ $A_N$

### 出力

答えを出力せよ。

* * *

### 入力例 1

3 4
2 5 0

### 出力例 1

10

-   $A_1 \mathbin{\mathrm{mod}} M = 2$
-   $(A_1+A_2) \mathbin{\mathrm{mod}} M = 3$
-   $(A_1+A_2+A_3) \mathbin{\mathrm{mod}} M = 3$
-   $A_2 \mathbin{\mathrm{mod}} M = 1$
-   $(A_2+A_3) \mathbin{\mathrm{mod}} M = 1$
-   $A_3 \mathbin{\mathrm{mod}} M = 0$

これらの総和の $10$ が答えです。外側の総和のあまりは取らないことに注意してください。

* * *

### 入力例 2

10 100
320 578 244 604 145 839 156 857 556 400

### 出力例 2

2736
