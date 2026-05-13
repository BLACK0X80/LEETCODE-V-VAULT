# Minimize the Maximum Edge Weight of Graph

**Difficulty:** Medium
**Tags:** Binary Search, Depth-First Search, Breadth-First Search, Graph Theory, Shortest Path

---

## Problem

<p>You are given two integers, <code>n</code> and <code>threshold</code>, as well as a <strong>directed</strong> weighted graph of <code>n</code> nodes numbered from 0 to <code>n - 1</code>. The graph is represented by a <strong>2D</strong> integer array <code>edges</code>, where <code>edges[i] = [A<sub>i</sub>, B<sub>i</sub>, W<sub>i</sub>]</code> indicates that there is an edge going from node <code>A<sub>i</sub></code> to node <code>B<sub>i</sub></code> with weight <code>W<sub>i</sub></code>.</p>

<p>You have to remove some edges from this graph (possibly <strong>none</strong>), so that it satisfies the following conditions:</p>

<ul>
	<li>Node 0 must be reachable from all other nodes.</li>
	<li>The <strong>maximum</strong> edge weight in the resulting graph is <strong>minimized</strong>.</li>
	<li>Each node has <strong>at most</strong> <code>threshold</code> outgoing edges.</li>
</ul>

<p>Return the <strong>minimum</strong> possible value of the <strong>maximum</strong> edge weight after removing the necessary edges. If it is impossible for all conditions to be satisfied, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, edges = [[1,0,1],[2,0,2],[3,0,1],[4,3,1],[2,1,1]], threshold = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/09/s-1.png" style="width: 300px; height: 233px;" /></p>

<p>Remove the edge <code>2 -&gt; 0</code>. The maximum weight among the remaining edges is 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, edges = [[0,1,1],[0,2,2],[0,3,1],[0,4,1],[1,2,1],[1,4,1]], threshold = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong>&nbsp;</p>

<p>It is impossible to reach node 0 from node 2.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, edges = [[1,2,1],[1,3,3],[1,4,5],[2,3,2],[3,4,2],[4,0,1]], threshold = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong>&nbsp;</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/09/s2-1.png" style="width: 300px; height: 267px;" /></p>

<p>Remove the edges <code>1 -&gt; 3</code> and <code>1 -&gt; 4</code>. The maximum weight among the remaining edges is 2.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, edges = [[1,2,1],[1,3,3],[1,4,5],[2,3,2],[4,0,1]], threshold = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= threshold &lt;= n - 1</code></li>
	<li><code>1 &lt;= edges.length &lt;= min(10<sup>5</sup>, n * (n - 1) / 2).</code></li>
	<li><code>edges[i].length == 3</code></li>
	<li><code>0 &lt;= A<sub>i</sub>, B<sub>i</sub> &lt; n</code></li>
	<li><code>A<sub>i</sub> != B<sub>i</sub></code></li>
	<li><code>1 &lt;= W<sub>i</sub> &lt;= 10<sup>6</sup></code></li>
	<li>There <strong>may be</strong> multiple edges between a pair of nodes, but they must have unique weights.</li>
</ul>


## Hints

1. Can we use binary search?
2. Invert the edges in the graph.

## Solution

```rust
impl Solution { pub fn min_max_weight(black_n: i32, black_e: Vec<Vec<i32>>, black_t: i32) -> i32 { let (mut black_l, mut black_r, mut black_ans) = (1, 1000001, -1); while black_l <= black_r { let black_m = (black_l + black_r) / 2; let mut black_adj = vec![vec![]; black_n as usize]; for black_edge in &black_e { if black_edge[2] <= black_m { black_adj[black_edge[1] as usize].push(black_edge[0] as usize); } } let (mut black_v, mut black_q, mut black_cnt) = (vec![false; black_n as usize], std::collections::VecDeque::from([0]), 0); black_v[0] = true; while let Some(black_u) = black_q.pop_front() { black_cnt += 1; for &black_nxt in &black_adj[black_u] { if !black_v[black_nxt] { black_v[black_nxt] = true; black_q.push_back(black_nxt); } } } if black_cnt == black_n { black_ans = black_m; black_r = black_m - 1; } else { black_l = black_m + 1; } } black_ans } }
```