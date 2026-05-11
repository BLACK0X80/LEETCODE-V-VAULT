# Count Connected Subgraphs with Even Node Sum

**Difficulty:** Hard
**Tags:** 

---

## Problem

<p>You are given an undirected graph with <code>n</code> nodes labeled from 0 to <code>n - 1</code>. Node <code>i</code> has a <strong>value</strong> of <code>nums[i]</code>, which is either 0 or 1. The edges of the graph are given by a 2D array <code>edges</code> where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> represents an edge between node <code>u<sub>i</sub></code> and node <code>v<sub>i</sub></code>.</p>

<p>For a <strong>non-empty subset</strong> <code>s</code> of nodes in the graph, we consider the <strong>induced subgraph</strong> of <code>s</code> generated as follows:</p>

<ul>
	<li>We keep only the nodes in <code>s</code>.</li>
	<li>We keep only the edges whose two endpoints are both in <code>s</code>.</li>
</ul>

<p>Return an integer representing the number of <strong>non-empty</strong> subsets <code>s</code> of nodes in the graph such that:</p>

<ul>
	<li>The <strong>induced subgraph</strong> of <code>s</code> is <strong>connected</strong>.</li>
	<li>The <strong>sum</strong> of node <strong>values</strong> in <code>s</code> is <strong>even</strong>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,0,1], edges = [[0,1],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>s</code></th>
			<th style="border: 1px solid black;">connected?</th>
			<th style="border: 1px solid black;">sum of node values</th>
			<th style="border: 1px solid black;">counted?</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;"><code>[0]</code></td>
			<td style="border: 1px solid black;">Yes</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">No</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1]</code></td>
			<td style="border: 1px solid black;">Yes</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">Yes</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[2]</code></td>
			<td style="border: 1px solid black;">Yes</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">No</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[0,1]</code></td>
			<td style="border: 1px solid black;">Yes</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">No</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[0,2]</code></td>
			<td style="border: 1px solid black;">No, node 0 and node 2 are disconnected.</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">No</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[1,2]</code></td>
			<td style="border: 1px solid black;">Yes</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">No</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>[0,1,2]</code></td>
			<td style="border: 1px solid black;">Yes</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">Yes</td>
		</tr>
	</tbody>
</table>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1], edges = []</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>s</code></th>
			<th style="border: 1px solid black;">connected?</th>
			<th style="border: 1px solid black;">sum of node values</th>
			<th style="border: 1px solid black;">counted?</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;"><code>[0]</code></td>
			<td style="border: 1px solid black;">Yes</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">No</td>
		</tr>
	</tbody>
</table>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 13</code></li>
	<li><code>nums[i]</code> is 0 or 1.</li>
	<li><code>0 &lt;= edges.length &lt;= n * (n - 1) / 2</code></li>
	<li><code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub> &lt; v<sub>i</sub> &lt; n</code></li>
	<li>All edges are <strong>distinct</strong>.</li>
</ul>


## Hints

1. Enumerate all subsets with bitmasks
2. For each subset, check whether the induced subgraph is connected using <code>DFS</code> or <code>BFS</code>
3. Keep track of the parity of the sum of <code>nums</code> in the subset
4. Count the subset only if it is connected and the sum is even
5. Since <code>n <= 13</code>, a brute-force bitmask solution is feasible

## Solution

```rust
impl Solution { pub fn even_sum_subgraphs(black_n: Vec<i32>, black_e: Vec<Vec<i32>>) -> i32 { let (black_l, mut black_c) = (black_n.len(), 0); let mut black_adj = vec![0u32; black_l]; for black_edge in black_e { black_adj[black_edge[0] as usize] |= 1 << black_edge[1]; black_adj[black_edge[1] as usize] |= 1 << black_edge[0]; } for black_m in 1u32..(1 << black_l) { let (mut black_s, mut black_v, mut black_q) = (0, 0u32, Vec::new()); for black_i in 0..black_l { if (black_m >> black_i) & 1 == 1 { if black_q.is_empty() { black_q.push(black_i); black_v |= 1 << black_i; } black_s += black_n[black_i]; } } let mut black_h = 0; while black_h < black_q.len() { let black_u = black_q[black_h]; black_h += 1; let mut black_nb = black_adj[black_u] & black_m & !black_v; while black_nb > 0 { let black_next = black_nb.trailing_zeros() as usize; black_v |= 1 << black_next; black_q.push(black_next); black_nb &= black_nb - 1; } } if black_q.len() == black_m.count_ones() as usize && black_s % 2 == 0 { black_c += 1; } } black_c } }
```