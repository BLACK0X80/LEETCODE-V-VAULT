# Number of Ways to Assign Edge Weights I

**Difficulty:** Medium
**Tags:** Math, Tree, Depth-First Search

---

## Problem

<p>There is an undirected tree with <code>n</code> nodes labeled from 1 to <code>n</code>, rooted at node 1. The tree is represented by a 2D integer array <code>edges</code> of length <code>n - 1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</p>

<p>Initially, all edges have a weight of 0. You must assign each edge a weight of either <strong>1</strong> or <strong>2</strong>.</p>

<p>The <strong>cost</strong> of a path between any two nodes <code>u</code> and <code>v</code> is the total weight of all edges in the path connecting them.</p>

<p>Select any one node <code>x</code> at the <strong>maximum</strong> depth. Return the number of ways to assign edge weights in the path from node 1 to <code>x</code> such that its total cost is <strong>odd</strong>.</p>

<p>Since the answer may be large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p><strong>Note:</strong> Ignore all edges <strong>not</strong> in the path from node 1 to <code>x</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/23/screenshot-2025-03-24-at-060006.png" style="width: 200px; height: 72px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The path from Node 1 to Node 2 consists of one edge (<code>1 &rarr; 2</code>).</li>
	<li>Assigning weight 1 makes the cost odd, while 2 makes it even. Thus, the number of valid assignments is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/03/23/screenshot-2025-03-24-at-055820.png" style="width: 220px; height: 207px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges = [[1,2],[1,3],[3,4],[3,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The maximum depth is 2, with nodes 4 and 5 at the same depth. Either node can be selected for processing.</li>
	<li>For example, the path from Node 1 to Node 4 consists of two edges (<code>1 &rarr; 3</code> and <code>3 &rarr; 4</code>).</li>
	<li>Assigning weights (1,2) or (2,1) results in an odd cost. Thus, the number of valid assignments is 2.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i] == [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>1 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n</code></li>
	<li><code>edges</code> represents a valid tree.</li>
</ul>


## Hints

1. Depth‑First Search (DFS) to compute the depth of each node from the root.
2. Find the maximum depth, <code>max_depth</code>.
3. The number of <code>2</code>s doesn’t affect parity; we only need an odd number of <code>1</code>s along the path.
4. The number of ways to choose an odd count of 1s among <code>max_depth</code> edges is <code>2^(max_depth-1)</code>.

## Solution

```rust
impl Solution { pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 { let mut black_adj = vec![vec![]; edges.len() + 2]; for e in &edges { black_adj[e[0] as usize].push(e[1]); black_adj[e[1] as usize].push(e[0]); } let (mut black_max_d, mut black_q) = (0, std::collections::VecDeque::from([(1, 0)])); let mut black_vis = vec![false; edges.len() + 2]; black_vis[1] = true; while let Some((u, d)) = black_q.pop_front() { black_max_d = black_max_d.max(d); for &v in &black_adj[u as usize] { if !black_vis[v as usize] { black_vis[v as usize] = true; black_q.push_back((v, d + 1)); } } } let mut black_res = 1i64; for _ in 0..(black_max_d - 1) { black_res = (black_res * 2) % 1_000_000_007; } black_res as i32 } }
```