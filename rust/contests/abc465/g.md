# G - Sum of Mex of Mod of Linear

Source: https://atcoder.jp/contests/abc465/tasks/abc465_g

配点 : $625$ 点

### 問題文

整数 $N,M,C,K$ と長さ $N$ の整数列 $A=(A_1,A_2,\ldots,A_N)$ が与えられます。

$Q$ 個のクエリが与えられるので、順に処理してください。$q$ 番目 $(1\le q\le Q)$ のクエリでは整数 $i_q,X_q$ が与えられるので、$A_{i_q}$ を $X_q$ に変更した後に以下の問題の答えを求めてください。

> $\displaystyle \sum_{k=0}^{K-1} \mathop{\mathrm{mex}}\limits_{1 \le i \le N} \lbrace (Ck+A_i)\bmod M\rbrace$ の値を求めてください。ただし、整数列 $B=(B_1,B_2,\ldots,B_N)$ に対し $\displaystyle \mathop{\mathrm{mex}}\limits_{1 \le i \le N} B_i$ は $B$ に含まれない最小の非負整数を表します。

### 制約

-   $1\le N\le 2\times 10^5$
-   $0\le C < M \le 10^9$
-   $1\le K\le 10^9$
-   $0\le A_i < M$
-   $1\le Q\le 2\times 10^5$
-   $1\le i_q \le N$
-   $0\le X_q < M$
-   入力される値は全て整数

* * *

### 入力

入力は以下の形式で標準入力から与えられる。

$N$ $M$ $C$ $K$
$A_1$ $A_2$ $\ldots$ $A_N$
$Q$
$i_1$ $X_1$
$i_2$ $X_2$
$\vdots$
$i_Q$ $X_Q$

### 出力

$Q$ 行出力せよ。

$q$ 行目 $(1\le q\le Q)$ には、$q$ 番目のクエリを処理した後の問題の答えを出力せよ。

* * *

### 入力例 1

2 3 1 2
0 2
3
2 2
1 1
2 1

### 出力例 1

3
1
0

$1$ 番目のクエリでは $A=(0,2)$ となります。求める値は $\displaystyle \text{mex}(\lbrace 0,2\rbrace)+\text{mex}(\lbrace 1,0\rbrace)=1+2=3$ です。

$2$ 番目のクエリでは $A=(1,2)$ となります。求める値は $\displaystyle \text{mex}(\lbrace 1,2\rbrace)+\text{mex}(\lbrace 2,0\rbrace)=0+1=1$ です。

$3$ 番目のクエリでは $A=(1,1)$ となります。求める値は $\displaystyle \text{mex}(\lbrace 1,1\rbrace)+\text{mex}(\lbrace 2,2\rbrace)=0+0=0$ です。

* * *

### 入力例 2

7 9 3 19
7 0 2 8 1 3 4
7
3 5
1 6
3 2
5 5
4 7
7 8
2 4

### 出力例 2

32
44
53
37
49
37
54
