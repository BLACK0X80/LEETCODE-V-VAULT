# Minimum Cost Path with Edge Reversals

**Difficulty:** Medium
**Tags:** Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>You are given a directed, weighted graph with <code>n</code> nodes labeled from 0 to <code>n - 1</code>, and an array <code>edges</code> where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]</code> represents a directed edge from node <code>u<sub>i</sub></code> to node <code>v<sub>i</sub></code> with cost <code>w<sub>i</sub></code>.</p>

<p>Each node <code>u<sub>i</sub></code> has a switch that can be used <strong>at most once</strong>: when you arrive at <code>u<sub>i</sub></code> and have not yet used its switch, you may activate it on one of its incoming edges <code>v<sub>i</sub> &rarr; u<sub>i</sub></code> reverse that edge to <code>u<sub>i</sub> &rarr; v<sub>i</sub></code> and <strong>immediately</strong> traverse it.</p>

<p>The reversal is only valid for that single move, and using a reversed edge costs <code>2 * w<sub>i</sub></code>.</p>

<p>Return the <strong>minimum</strong> total cost to travel from node 0 to node <code>n - 1</code>. If it is not possible, return -1.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[0,1,3],[3,1,1],[2,3,4],[0,2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation: </strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2025/05/07/e1drawio.png" style="width: 171px; height: 111px;" /></strong></p>

<ul>
	<li>Use the path <code>0 &rarr; 1</code> (cost 3).</li>
	<li>At node 1 reverse the original edge <code>3 &rarr; 1</code> into <code>1 &rarr; 3</code> and traverse it at cost <code>2 * 1 = 2</code>.</li>
	<li>Total cost is <code>3 + 2 = 5</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[0,2,1],[2,1,1],[1,3,1],[2,3,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>No reversal is needed. Take the path <code>0 &rarr; 2</code> (cost 1), then <code>2 &rarr; 1</code> (cost 1), then <code>1 &rarr; 3</code> (cost 1).</li>
	<li>Total cost is <code>1 + 1 + 1 = 3</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= edges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, w<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>1 &lt;= w<sub>i</sub> &lt;= 1000</code></li>
</ul>


## Hints

1. Do we only need to reverse at most one edge for each node? If so, can we add reversed edges for each node and use the one that helps in the shortest path?
2. Add reverse edges: <code>{u, v, w}</code> -> <code>{v, u, 2 * w}</code>, and use Dijkstra.

## Solution

```rust
impl Solution { pub fn min_cost(black_n: i32, black_e: Vec<Vec<i32>>) -> i32 { let black_n = black_n as usize; let mut black_adj = vec![vec![]; black_n]; for black_edge in black_e { black_adj[black_edge[0] as usize].push((black_edge[1] as usize, black_edge[2], false)); black_adj[black_edge[1] as usize].push((black_edge[0] as usize, black_edge[2] * 2, true)); } let mut black_dist = vec![i32::MAX; black_n]; let mut black_pq = std::collections::BinaryHeap::from([(0, 0)]); black_dist[0] = 0; while let Some((black_d, black_u)) = black_pq.pop() { let black_d = -black_d; if black_d > black_dist[black_u] { continue; } if black_u == black_n - 1 { return black_d; } for &(black_v, black_w, _) in &black_adj[black_u] { if black_dist[black_u] != i32::MAX && black_dist[black_v] > black_d + black_w { black_dist[black_v] = black_d + black_w; black_pq.push((-black_dist[black_v], black_v)); } } } if black_dist[black_n - 1] == i32::MAX { -1 } else { black_dist[black_n - 1] } } }
```