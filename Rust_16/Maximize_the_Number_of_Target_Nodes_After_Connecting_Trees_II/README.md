# Maximize the Number of Target Nodes After Connecting Trees II

**Difficulty:** Hard
**Tags:** Tree, Depth-First Search, Breadth-First Search

---

## Problem

<p>There exist two <strong>undirected </strong>trees with <code>n</code> and <code>m</code> nodes, labeled from <code>[0, n - 1]</code> and <code>[0, m - 1]</code>, respectively.</p>

<p>You are given two 2D integer arrays <code>edges1</code> and <code>edges2</code> of lengths <code>n - 1</code> and <code>m - 1</code>, respectively, where <code>edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the first tree and <code>edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> in the second tree.</p>

<p>Node <code>u</code> is <strong>target</strong> to node <code>v</code> if the number of edges on the path from <code>u</code> to <code>v</code> is even.&nbsp;<strong>Note</strong> that a node is <em>always</em> <strong>target</strong> to itself.</p>

<p>Return an array of <code>n</code> integers <code>answer</code>, where <code>answer[i]</code> is the <strong>maximum</strong> possible number of nodes that are <strong>target</strong> to node <code>i</code> of the first tree if you had to connect one node from the first tree to another node in the second tree.</p>

<p><strong>Note</strong> that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[8,7,7,8,8]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For <code>i = 0</code>, connect node 0 from the first tree to node 0 from the second tree.</li>
	<li>For <code>i = 1</code>, connect node 1 from the first tree to node 4 from the second tree.</li>
	<li>For <code>i = 2</code>, connect node 2 from the first tree to node 7 from the second tree.</li>
	<li>For <code>i = 3</code>, connect node 3 from the first tree to node 0 from the second tree.</li>
	<li>For <code>i = 4</code>, connect node 4 from the first tree to node 4 from the second tree.</li>
</ul>
<img alt="" src="https://assets.leetcode.com/uploads/2024/09/24/3982-1.png" style="width: 600px; height: 169px;" /></div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,6,6,6,6]</span></p>

<p><strong>Explanation:</strong></p>

<p>For every <code>i</code>, connect node <code>i</code> of the first tree with any node of the second tree.</p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/09/24/3928-2.png" style="height: 281px; width: 500px;" /></div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n, m &lt;= 10<sup>5</sup></code></li>
	<li><code>edges1.length == n - 1</code></li>
	<li><code>edges2.length == m - 1</code></li>
	<li><code>edges1[i].length == edges2[i].length == 2</code></li>
	<li><code>edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; m</code></li>
	<li>The input is generated such that <code>edges1</code> and <code>edges2</code> represent valid trees.</li>
</ul>


## Hints

1. Compute an array <code>even</code> where <code>even[u]</code> is the number of nodes at an even distance from node <code>u</code>, for every <code>u</code> of the first tree.
2. Compute an array <code>odd</code> where <code>odd[u]</code> is the number of nodes at an odd distance from node <code>u</code>, for every <code>u</code> of the second tree.
3. <code>answer[i] = even[i]+ max(odd[1], odd[2], …, odd[m - 1])</code>

## Solution

```rust
impl Solution {
    pub fn max_target_nodes(black_e1: Vec<Vec<i32>>, black_e2: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_e1.len() + 1;
        let black_m = black_e2.len() + 1;

        let black_get_parity = |black_size: usize, black_edges: Vec<Vec<i32>>| -> (Vec<usize>, [i32; 2]) {
            let mut black_adj = vec![vec![]; black_size];
            for black_e in black_edges {
                black_adj[black_e[0] as usize].push(black_e[1] as usize);
                black_adj[black_e[1] as usize].push(black_e[0] as usize);
            }
            let mut black_p = vec![0; black_size];
            let mut black_c = [0, 0];
            let mut black_q = std::collections::VecDeque::from([(0, 0)]);
            let mut black_vis = vec![false; black_size];
            black_vis[0] = true;

            while let Some((black_u, black_color)) = black_q.pop_front() {
                black_p[black_u] = black_color;
                black_c[black_color] += 1;
                for &black_v in &black_adj[black_u] {
                    if !black_vis[black_v] {
                        black_vis[black_v] = true;
                        black_q.push_back((black_v, 1 - black_color));
                    }
                }
            }
            (black_p, black_c)
        };

        let (black_p1, black_c1) = black_get_parity(black_n, black_e1);
        let (_, black_c2) = black_get_parity(black_m, black_e2);

        let black_max_tree2 = black_c2[0].max(black_c2[1]);
        let mut black_res = vec![0; black_n];

        for black_i in 0..black_n {
            black_res[black_i] = black_c1[black_p1[black_i]] + black_max_tree2;
        }

        black_res
    }
}
```