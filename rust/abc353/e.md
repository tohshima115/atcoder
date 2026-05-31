# E - Yet Another Sigma Problem

Source: https://atcoder.jp/contests/abc353/tasks/abc353_e

配点 : $500$ 点

### 問題文

文字列 $x,y$ に対して $f(x,y)$ を以下で定義します。

-   $x,y$ の最長共通接頭辞の長さを $f(x,y)$ とする。

英小文字からなる $N$ 個の文字列 $(S_1,\ldots,S_N)$ が与えられます。次の式の値を求めてください。

$\displaystyle \sum_{i=1}^{N-1}\sum_{j=i+1}^N f(S_i,S_j)$

  

### 制約

-   $2\leq N\leq 3\times 10^5$
-   $S_i$ は英小文字からなる文字列
-   $1\leq |S_i|$
-   $|S_1|+|S_2|+\ldots+|S_N|\leq 3\times 10^5$
-   入力される数値は全て整数

* * *

### 入力

入力は以下の形式で標準入力から与えられる。

$N$ 
$S_1$ $\ldots$ $S_N$

### 出力

答えを出力せよ。

* * *

### 入力例 1

3
ab abc arc

### 出力例 1

4

-   $f(S_1,S_2)=2$
-   $f(S_1,S_3)=1$
-   $f(S_2,S_3)=1$

なので、答えは $f(S_1,S_2)+f(S_1,S_3)+f(S_2,S_3) = 4$ です。

* * *

### 入力例 2

11
ab bb aaa bba baba babb aaaba aabbb a a b

### 出力例 2

32
