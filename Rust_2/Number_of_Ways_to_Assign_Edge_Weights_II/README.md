# Number of Ways to Assign Edge Weights II

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Bit Manipulation, Tree, Depth-First Search

---

## Problem

<p>There is an undirected tree with <code>n</code> nodes labeled from 1 to <code>n</code>, rooted at node 1. The tree is represented by a 2D integer array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</p>

<p>Initially, all edges have a weight of 0. You must assign each edge a weight of either <strong>1</strong> or <strong>2</strong>.</p>

<p>The <strong>cost</strong> of a path between any two nodes <code>u</code> and <code>v</code> is the total weight of all edges in the path connecting them.</p>

<p>You are given a 2D integer array <code>queries</code>. For each <code>queries[i] = [u<sub>i</sub>, v<sub>i</sub>]</code>, determine the number of ways to assign weights to edges <strong>in the path</strong> such that the cost of the path between <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> is <strong>odd</strong>.</p>

<p>Return an array <code>answer</code>, where <code>answer[i]</code> is the number of valid assignments for <code>queries[i]</code>.</p>

<p>Since the answer may be large, apply <strong>modulo</strong> <code>10<sup>9</sup> + 7</code> to each <code>answer[i]</code>.</p>

<p><strong>Note:</strong> For each query, disregard all edges <strong>not</strong> in the path between node <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><img src="https://assets.leetcode.com/uploads/2025/03/23/screenshot-2025-03-24-at-060006.png" style="height: 72px; width: 200px;" /></p>

<p><strong>Input:</strong> <span class="example-io">edges = [[1,2]], queries = [[1,1],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,1]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Query <code>[1,1]</code>: The path from Node 1 to itself consists of no edges, so the cost is 0. Thus, the number of valid assignments is 0.</li>
	<li>Query <code>[1,2]</code>: The path from Node 1 to Node 2 consists of one edge (<code>1 &rarr; 2</code>). Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/23/screenshot-2025-03-24-at-055820.png" style="height: 207px; width: 220px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[1,2],[1,3],[3,4],[3,5]], queries = [[1,4],[3,4],[2,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[2,1,4]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Query <code>[1,4]</code>: The path from Node 1 to Node 4 consists of two edges (<code>1 &rarr; 3</code> and <code>3 &rarr; 4</code>). Assigning weights (1,2) or (2,1) results in an odd cost. Thus, the number of valid assignments is 2.</li>
	<li>Query <code>[3,4]</code>: The path from Node 3 to Node 4 consists of one edge (<code>3 &rarr; 4</code>). Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.</li>
	<li>Query <code>[2,5]</code>: The path from Node 2 to Node 5 consists of three edges (<code>2 &rarr; 1, 1 &rarr; 3</code>, and <code>3 &rarr; 5</code>). Assigning (1,2,2), (2,1,2), (2,2,1), or (1,1,1) makes the cost odd. Thus, the number of valid assignments is 4.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] == [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>queries[i] == [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>1 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n</code></li>
	<li><code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. Dynamic programming with states <code>chainLength</code> and <code>sumParity</code>.
2. Use Lowest Common Ancestor to find the distance between any two nodes quickly in <code>O(logn)</code>.

## Solution

```rust
impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut black1 = vec![vec![]; n + 1];
        for e in edges {
            black1[e[0] as usize].push(e[1] as usize);
            black1[e[1] as usize].push(e[0] as usize);
        }
        let mut black2 = vec![0; n + 1];
        let mut black3 = vec![vec![0; 20]; n + 1];
        fn dfs(u: usize, p: usize, d: i32, g: &Vec<Vec<usize>>, dep: &mut Vec<i32>, par: &mut Vec<Vec<usize>>) {
            dep[u] = d;
            par[u][0] = p;
            for i in 1..20 { par[u][i] = par[par[u][i - 1]][i - 1]; }
            for &v in &g[u] { if v != p { dfs(v, u, d + 1, g, dep, par); } }
        }
        dfs(1, 1, 0, &black1, &mut black2, &mut black3);
        let black4 = |mut u: usize, mut v: usize, dep: &Vec<i32>, par: &Vec<Vec<usize>>| -> usize {
            if dep[u] < dep[v] { std::mem::swap(&mut u, &mut v); }
            for i in (0..20).rev() { if dep[par[u][i]] >= dep[v] { u = par[u][i]; } }
            if u == v { return u; }
            for i in (0..20).rev() { if par[u][i] != par[v][i] { u = par[u][i]; v = par[v][i]; } }
            par[u][0]
        };
        let mut black5 = vec![1i64; n + 1];
        let m = 1_000_000_007;
        for i in 1..n { black5[i] = (black5[i - 1] * 2) % m; }
        queries.into_iter().map(|q| {
            let (u, v) = (q[0] as usize, q[1] as usize);
            if u == v { return 0; }
            let lca = black4(u, v, &black2, &black3);
            let dist = (black2[u] + black2[v] - 2 * black2[lca]) as usize;
            black5[dist - 1] as i32
        }).collect()
    }
}
```