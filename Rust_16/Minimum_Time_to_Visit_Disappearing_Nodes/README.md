# Minimum Time to Visit Disappearing Nodes

**Difficulty:** Medium
**Tags:** Array, Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>There is an undirected graph of <code>n</code> nodes. You are given a 2D array <code>edges</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, length<sub>i</sub>]</code> describes an edge between node <code>u<sub>i</sub></code> and node <code>v<sub>i</sub></code> with a traversal time of <code>length<sub>i</sub></code> units.</p>

<p>Additionally, you are given an array <code>disappear</code>, where <code>disappear[i]</code> denotes the time when the node <code>i</code> disappears from the graph and you won&#39;t be able to visit it.</p>

<p><strong>Note</strong>&nbsp;that the graph might be <em>disconnected</em> and might contain <em>multiple edges</em>.</p>

<p>Return the array <code>answer</code>, with <code>answer[i]</code> denoting the <strong>minimum</strong> units of time required to reach node <code>i</code> from node 0. If node <code>i</code> is <strong>unreachable</strong> from node 0 then <code>answer[i]</code> is <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1,2],[1,2,1],[0,2,4]], disappear = [1,1,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,-1,4]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/11/output-onlinepngtools.png" style="width: 350px; height: 210px;" /></p>

<p>We are starting our journey from node 0, and our goal is to find the minimum time required to reach each node before it disappears.</p>

<ul>
	<li>For node 0, we don&#39;t need any time as it is our starting point.</li>
	<li>For node 1, we need at least 2 units of time to traverse <code>edges[0]</code>. Unfortunately, it disappears at that moment, so we won&#39;t be able to visit it.</li>
	<li>For node 2, we need at least 4 units of time to traverse <code>edges[2]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1,2],[1,2,1],[0,2,4]], disappear = [1,3,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,2,3]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/11/output-onlinepngtools-1.png" style="width: 350px; height: 210px;" /></p>

<p>We are starting our journey from node 0, and our goal is to find the minimum time required to reach each node before it disappears.</p>

<ul>
	<li>For node 0, we don&#39;t need any time as it is the starting point.</li>
	<li>For node 1, we need at least 2 units of time to traverse <code>edges[0]</code>.</li>
	<li>For node 2, we need at least 3 units of time to traverse <code>edges[0]</code> and <code>edges[1]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2, edges = [[0,1,1]], disappear = [1,1]</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,-1]</span></p>

<p><strong>Explanation:</strong></p>

<p>Exactly when we reach node 1, it disappears.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= edges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges[i] == [u<sub>i</sub>, v<sub>i</sub>, length<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>1 &lt;= length<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
	<li><code>disappear.length == n</code></li>
	<li><code>1 &lt;= disappear[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Use Dijkstra’s algorithm, but only visit nodes if you can reach them before disappearance.

## Solution

```rust
impl Solution { pub fn minimum_time(black_n: i32, black_e: Vec<Vec<i32>>, black_d: Vec<i32>) -> Vec<i32> { let black_n = black_n as usize; let mut black_adj = vec![vec![]; black_n]; for black_edge in black_e { black_adj[black_edge[0] as usize].push((black_edge[1] as usize, black_edge[2])); black_adj[black_edge[1] as usize].push((black_edge[0] as usize, black_edge[2])); } let mut black_res = vec![-1; black_n]; let mut black_pq = std::collections::BinaryHeap::from([(std::cmp::Reverse(0), 0)]); while let Some((std::cmp::Reverse(black_t), black_u)) = black_pq.pop() { if black_res[black_u] != -1 { continue; } black_res[black_u] = black_t; for &(black_v, black_l) in &black_adj[black_u] { let black_nt = black_t + black_l; if black_res[black_v] == -1 && black_nt < black_d[black_v] { black_pq.push((std::cmp::Reverse(black_nt), black_v)); } } } black_res } }
```