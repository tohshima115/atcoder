// C-11: 連結成分の個数（BFS/DFS）
// 認識トリガー: 「グループ分け」「つながりのかたまりの数」→ 連結成分
// 無向は辺を両方向 push。未訪問の頂点から探索を始めるたびに comp += 1。BFS推奨

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }
    let mut adj = vec![vec![]; n + 1];
    for &(u, v) in &edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let _ = adj;
    let _q: VecDeque<usize> = VecDeque::new();
    // TODO
}
