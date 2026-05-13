# Maximum Subgraph Score in a Tree

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Tree, Depth-First Search

---

## Problem

<p>You are given an <strong>undirected tree</strong> with <code>n</code> nodes, numbered from 0 to <code>n - 1</code>. It is represented by a 2D integer array <code>edges</code>​​​​​​​ of length <code>n - 1</code>, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree.</p>

<p>You are also given an integer array <code>good</code> of length <code>n</code>, where <code>good[i]</code> is 1 if the <code>i<sup>th</sup></code> node is good, and 0 if it is bad.</p>

<p>Define the <strong>score</strong> of a <strong>subgraph</strong> as the number of good nodes minus the number of bad nodes in that subgraph.</p>

<p>For each node <code>i</code>, find the <strong>maximum</strong> possible score among all <strong>connected subgraphs</strong> that contain node <code>i</code>.</p>

<p>Return an array of <code>n</code> integers where the <code>i<sup>th</sup></code> element is the <strong>maximum</strong> score for node <code>i</code>.</p>

<p>A <strong>subgraph</strong> is a graph whose vertices and edges are subsets of the original graph.</p>

<p>A <strong>connected subgraph</strong> is a subgraph in which every pair of its vertices is reachable from one another using only its edges.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="Tree Example 1" src="https://assets.leetcode.com/uploads/2025/11/17/tree1fixed.png" style="width: 271px; height: 51px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1],[1,2]], good = [1,0,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1,1,1]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Green nodes are good and red nodes are bad.</li>
	<li>For each node, the best connected subgraph containing it is the whole tree, which has 2 good nodes and 1 bad node, resulting in a score of 1.</li>
	<li>Other connected subgraphs containing a node may have the same score.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="Tree Example 2" src="https://assets.leetcode.com/uploads/2025/11/17/tree2.png" style="width: 211px; height: 231px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, edges = [[1,0],[1,2],[1,3],[3,4]], good = [0,1,0,1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,3,2,3,3]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Node 0: The best connected subgraph consists of nodes <code>0, 1, 3, 4</code>, which has 3 good nodes and 1 bad node, resulting in a score of <code>3 - 1 = 2</code>.</li>
	<li>Nodes 1, 3, and 4: The best connected subgraph consists of nodes <code>1, 3, 4</code>, which has 3 good nodes, resulting in a score of 3.</li>
	<li>Node 2: The best connected subgraph consists of nodes <code>1, 2, 3, 4</code>, which has 3 good nodes and 1 bad node, resulting in a score of <code>3 - 1 = 2</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<p><img alt="Tree Example 3" src="https://assets.leetcode.com/uploads/2025/11/17/tree3.png" style="width: 161px; height: 51px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2, edges = [[0,1]], good = [0,0]</span></p>

<p><strong>Output:</strong> <span class="example-io">[-1,-1]</span></p>

<p><strong>Explanation:</strong></p>

<p>For each node, including the other node only adds another bad node, so the best score for both nodes is -1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>good.length == n</code></li>
	<li><code>0 &lt;= good[i] &lt;= 1</code></li>
	<li>The input is generated such that <code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. Use rerooting dynamic programming

## Solution

```rust
impl Solution {
    pub fn max_subgraph_score(black_n: i32, black_edges: Vec<Vec<i32>>, black_good: Vec<i32>) -> Vec<i32> {
        let black_n_idx = black_n as usize;
        let mut black_adj = vec![Vec::new(); black_n_idx];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let black_scores: Vec<i32> = black_good.iter().map(|&black_g| if black_g == 1 { 1 } else { -1 }).collect();
        let mut black_down = vec![0; black_n_idx];
        let mut black_res = vec![0; black_n_idx];

        Self::black_dfs_down(0, usize::MAX, &black_adj, &black_scores, &mut black_down);
        Self::black_dfs_up(0, usize::MAX, 0, &black_adj, &black_scores, &black_down, &mut black_res);

        black_res
    }

    fn black_dfs_down(black_u: usize, black_p: usize, black_adj: &Vec<Vec<usize>>, black_scores: &Vec<i32>, black_down: &mut Vec<i32>) {
        let mut black_curr = black_scores[black_u];
        for &black_v in &black_adj[black_u] {
            if black_v == black_p { continue; }
            Self::black_dfs_down(black_v, black_u, black_adj, black_scores, black_down);
            if black_down[black_v] > 0 { black_curr += black_down[black_v]; }
        }
        black_down[black_u] = black_curr;
    }

    fn black_dfs_up(black_u: usize, black_p: usize, black_up_val: i32, black_adj: &Vec<Vec<usize>>, black_scores: &Vec<i32>, black_down: &Vec<i32>, black_res: &mut Vec<i32>) {
        black_res[black_u] = black_down[black_u] + if black_up_val > 0 { black_up_val } else { 0 };
        for &black_v in &black_adj[black_u] {
            if black_v == black_p { continue; }
            let mut black_next_up = black_res[black_u];
            if black_down[black_v] > 0 { black_next_up -= black_down[black_v]; }
            Self::black_dfs_up(black_v, black_u, black_next_up, black_adj, black_scores, black_down, black_res);
        }
    }
}
```