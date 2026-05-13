# Maximize Sum of Weights after Edge Removals

**Difficulty:** Hard
**Tags:** Dynamic Programming, Tree, Depth-First Search, Sorting

---

## Problem

<p>There exists an <strong>undirected</strong> tree with <code>n</code> nodes numbered <code>0</code> to <code>n - 1</code>. You are given a 2D integer array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> with weight <code>w<sub>i</sub></code> in the tree.</p>

<p>Your task is to remove <em>zero or more</em> edges such that:</p>

<ul>
	<li>Each node has an edge with <strong>at most</strong> <code>k</code> other nodes, where <code>k</code> is given.</li>
	<li>The sum of the weights of the remaining edges is <strong>maximized</strong>.</li>
</ul>

<p>Return the <strong>maximum </strong>possible sum of weights for the remaining edges after making the necessary removals.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[0,1,4],[0,2,2],[2,3,12],[2,4,6]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">22</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/10/30/test1drawio.png" style="width: 250px; height: 250px;" /></p>

<ul>
	<li>Node 2 has edges with 3 other nodes. We remove the edge <code>[0, 2, 2]</code>, ensuring that no node has edges with more than <code>k = 2</code> nodes.</li>
	<li>The sum of weights is 22, and we can&#39;t achieve a greater sum. Thus, the answer is 22.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[0,1,5],[1,2,10],[0,3,15],[3,4,20],[3,5,5],[0,6,10]], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">65</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Since no node has edges connecting it to more than <code>k = 3</code> nodes, we don&#39;t remove any edges.</li>
	<li>The sum of weights is 65. Thus, the answer is 65.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= n - 1</code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i].length == 3</code></li>
	<li><code><font face="monospace">0 &lt;= edges[i][0] &lt;= n - 1</font></code></li>
	<li><code><font face="monospace">0 &lt;= edges[i][1] &lt;= n - 1</font></code></li>
	<li><code><font face="monospace">1 &lt;= edges[i][2] &lt;= 10<sup>6</sup></font></code></li>
	<li>The input is generated such that <code>edges</code> form a valid tree.</li>
</ul>


## Hints

1. Can we use DFS based approach here?
2. For each edge, find two sums: one including the edge and one excluding it.

## Solution

```rust
impl Solution {
    pub fn maximize_sum_of_weights(black_edges: Vec<Vec<i32>>, black_k: i32) -> i64 {
        let black_n = black_edges.len() + 1;
        let mut black_adj = vec![vec![]; black_n];
        for e in black_edges {
            black_adj[e[0] as usize].push((e[1] as usize, e[2] as i64));
            black_adj[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }

        fn black_dfs(u: usize, p: usize, k: i32, adj: &Vec<Vec<(usize, i64)>>) -> (i64, i64) {
            let mut black_base_sum = 0;
            let mut black_diffs = vec![];

            for &(v, w) in &adj[u] {
                if v == p { continue; }
                let (black_v_k, black_v_k1) = black_dfs(v, u, k, adj);
                black_base_sum += black_v_k;
                let black_gain = w + black_v_k1 - black_v_k;
                if black_gain > 0 { black_diffs.push(black_gain); }
            }

            black_diffs.sort_unstable_by(|a, b| b.cmp(a));
            let black_d_sum: i64 = black_diffs.iter().take(k as usize).sum();
            let black_d_sum_minus: i64 = black_diffs.iter().take(k as usize - 1).sum();

            (black_base_sum + black_d_sum, black_base_sum + black_d_sum_minus)
        }

        black_dfs(0, 0, black_k, &black_adj).0
    }
}
```