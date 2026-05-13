# Minimum Time to Reach Destination in Directed Graph

**Difficulty:** Medium
**Tags:** Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>You are given an integer <code>n</code> and a <strong>directed</strong> graph with <code>n</code> nodes labeled from 0 to <code>n - 1</code>. This is represented by a 2D array <code>edges</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, start<sub>i</sub>, end<sub>i</sub>]</code> indicates an edge from node <code>u<sub>i</sub></code> to <code>v<sub>i</sub></code> that can <strong>only</strong> be used at any integer time <code>t</code> such that <code>start<sub>i</sub> &lt;= t &lt;= end<sub>i</sub></code>.</p>

<p>You start at node 0 at time 0.</p>

<p>In one unit of time, you can either:</p>

<ul>
	<li>Wait at your current node without moving, or</li>
	<li>Travel along an outgoing edge from your current node if the current time <code>t</code> satisfies <code>start<sub>i</sub> &lt;= t &lt;= end<sub>i</sub></code>.</li>
</ul>

<p>Return the <strong>minimum</strong> time required to reach node <code>n - 1</code>. If it is impossible, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[0,1,0,1],[1,2,2,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/06/05/screenshot-2025-06-06-at-004535.png" style="width: 150px; height: 141px;" /></p>

<p>The optimal path is:</p>

<ul>
	<li>At time <code>t = 0</code>, take the edge <code>(0 &rarr; 1)</code> which is available from 0 to 1. You arrive at node 1 at time <code>t = 1</code>, then wait until <code>t = 2</code>.</li>
	<li>At time <code>t = <code>2</code></code>, take the edge <code>(1 &rarr; 2)</code> which is available from 2 to 5. You arrive at node 2 at time 3.</li>
</ul>

<p>Hence, the minimum time to reach node 2 is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[0,1,0,3],[1,3,7,8],[0,2,1,5],[2,3,4,7]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/06/05/screenshot-2025-06-06-at-004757.png" style="width: 170px; height: 219px;" /></p>

<p>The optimal path is:</p>

<ul>
	<li>Wait at node 0 until time <code>t = 1</code>, then take the edge <code>(0 &rarr; 2)</code> which is available from 1 to 5. You arrive at node 2 at <code>t = 2</code>.</li>
	<li>Wait at node 2 until time <code>t = 4</code>, then take the edge <code>(2 &rarr; 3)</code> which is available from 4 to 7. You arrive at node 3 at <code>t = 5</code>.</li>
</ul>

<p>Hence, the minimum time to reach node 3 is 5.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, edges = [[1,0,1,3],[1,2,3,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/06/05/screenshot-2025-06-06-at-004914.png" style="width: 150px; height: 145px;" /></p>

<ul>
	<li>Since there is no outgoing edge from node 0, it is impossible to reach node 2. Hence, the output is -1.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= edges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges[i] == [u<sub>i</sub>, v<sub>i</sub>, start<sub>i</sub>, end<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>u<sub>i</sub> != v<sub>i</sub></code></li>
	<li><code>0 &lt;= start<sub>i</sub> &lt;= end<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Use the <code>Dijkstra</code> algorithm over states (node, time).
2. At node <code>u</code> with current time <code>t</code>, you can only use an edge <code>[u, v, start, end]</code> if <code>t <= end</code>.
3. If <code>t < start</code>, wait until <code>start</code>, then traverse (arriving at <code>start + 1</code>).

## Solution

```rust
impl Solution { pub fn min_time(black_n: i32, black_e: Vec<Vec<i32>>) -> i32 { let black_n = black_n as usize; let mut black_adj = vec![vec![]; black_n]; for black_edge in black_e { black_adj[black_edge[0] as usize].push((black_edge[1] as usize, black_edge[2], black_edge[3])); } let mut black_dist = vec![i64::MAX; black_n]; let mut black_pq = std::collections::BinaryHeap::from([(0i64, 0usize)]); black_dist[0] = 0; while let Some((black_d, black_u)) = black_pq.pop() { let black_d = -black_d; if black_u == black_n - 1 { return black_d as i32; } if black_d > black_dist[black_u] { continue; } for &(black_v, black_s, black_e) in &black_adj[black_u] { let black_start_time = black_d.max(black_s as i64); if black_start_time <= black_e as i64 { let black_reach = black_start_time + 1; if black_reach < black_dist[black_v] { black_dist[black_v] = black_reach; black_pq.push((-black_reach, black_v)); } } } } -1 } }
```