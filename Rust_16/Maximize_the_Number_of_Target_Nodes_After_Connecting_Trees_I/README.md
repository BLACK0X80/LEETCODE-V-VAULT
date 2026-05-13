# Maximize the Number of Target Nodes After Connecting Trees I

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search

---

## Problem

<p>There exist two <strong>undirected </strong>trees with <code>n</code> and <code>m</code> nodes, with <strong>distinct</strong> labels in ranges <code>[0, n - 1]</code> and <code>[0, m - 1]</code>, respectively.</p>

<p>You are given two 2D integer arrays <code>edges1</code> and <code>edges2</code> of lengths <code>n - 1</code> and <code>m - 1</code>, respectively, where <code>edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the first tree and <code>edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> in the second tree. You are also given an integer <code>k</code>.</p>

<p>Node <code>u</code> is <strong>target</strong> to node <code>v</code> if the number of edges on the path from <code>u</code> to <code>v</code> is less than or equal to <code>k</code>. <strong>Note</strong> that a node is <em>always</em> <strong>target</strong> to itself.</p>

<p>Return an array of <code>n</code> integers <code>answer</code>, where <code>answer[i]</code> is the <strong>maximum</strong> possible number of nodes <strong>target</strong> to node <code>i</code> of the first tree if you have to connect one node from the first tree to another node in the second tree.</p>

<p><strong>Note</strong> that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[9,7,9,8,8]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>For <code>i = 0</code>, connect node 0 from the first tree to node 0 from the second tree.</li>
	<li>For <code>i = 1</code>, connect node 1 from the first tree to node 0 from the second tree.</li>
	<li>For <code>i = 2</code>, connect node 2 from the first tree to node 4 from the second tree.</li>
	<li>For <code>i = 3</code>, connect node 3 from the first tree to node 4 from the second tree.</li>
	<li>For <code>i = 4</code>, connect node 4 from the first tree to node 4 from the second tree.</li>
</ul>
<img alt="" src="https://assets.leetcode.com/uploads/2024/09/24/3982-1.png" style="width: 600px; height: 169px;" /></div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">[6,3,3,3,3]</span></p>

<p><strong>Explanation:</strong></p>

<p>For every <code>i</code>, connect node <code>i</code> of the first tree with any node of the second tree.</p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/09/24/3928-2.png" style="height: 281px; width: 500px;" /></div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n, m &lt;= 1000</code></li>
	<li><code>edges1.length == n - 1</code></li>
	<li><code>edges2.length == m - 1</code></li>
	<li><code>edges1[i].length == edges2[i].length == 2</code></li>
	<li><code>edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; m</code></li>
	<li>The input is generated such that <code>edges1</code> and <code>edges2</code> represent valid trees.</li>
	<li><code>0 &lt;= k &lt;= 1000</code></li>
</ul>


## Hints

1. For each node <code>u</code> in the first tree, find the number of nodes at a distance of at most <code>k</code> from node <code>u</code>.
2. For each node <code>v</code> in the second tree, find the number of nodes at a distance of at most <code>k - 1</code> from node <code>v</code>.

## Solution

```rust
impl Solution { pub fn max_target_nodes(black_e1: Vec<Vec<i32>>, black_e2: Vec<Vec<i32>>, black_k: i32) -> Vec<i32> { let black_f = |black_e: Vec<Vec<i32>>, black_lim: i32| { if black_lim < 0 { return vec![0; black_e.len() + 1]; } let black_sz = black_e.len() + 1; let mut black_adj = vec![vec![]; black_sz]; for black_edge in black_e { black_adj[black_edge[0] as usize].push(black_edge[1] as usize); black_adj[black_edge[1] as usize].push(black_edge[0] as usize); } (0..black_sz).map(|black_i| { let (mut black_q, mut black_v, mut black_c) = (std::collections::VecDeque::from([(black_i, 0)]), vec![false; black_sz], 0); black_v[black_i] = true; while let Some((black_u, black_d)) = black_q.pop_front() { black_c += 1; if black_d < black_lim { for &black_nxt in &black_adj[black_u] { if !black_v[black_nxt] { black_v[black_nxt] = true; black_q.push_back((black_nxt, black_d + 1)); } } } } black_c }).collect::<Vec<i32>>() }; let (black_c1, black_c2) = (black_f(black_e1, black_k), black_f(black_e2, black_k - 1)); let black_m2 = *black_c2.iter().max().unwrap_or(&0); black_c1.into_iter().map(|black_x| black_x + black_m2).collect() } }
```