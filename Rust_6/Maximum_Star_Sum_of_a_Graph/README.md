# Maximum Star Sum of a Graph

**Difficulty:** Medium
**Tags:** Array, Greedy, Graph Theory, Sorting, Heap (Priority Queue)

---

## Problem

<p>There is an undirected graph consisting of <code>n</code> nodes numbered from <code>0</code> to <code>n - 1</code>. You are given a <strong>0-indexed</strong> integer array <code>vals</code> of length <code>n</code> where <code>vals[i]</code> denotes the value of the <code>i<sup>th</sup></code> node.</p>

<p>You are also given a 2D integer array <code>edges</code> where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> denotes that there exists an <strong>undirected</strong> edge connecting nodes <code>a<sub>i</sub></code> and <code>b<sub>i.</sub></code></p>

<p>A <strong>star graph</strong> is a subgraph of the given graph having a center node containing <code>0</code> or more neighbors. In other words, it is a subset of edges of the given graph such that there exists a common node for all edges.</p>

<p>The image below shows star graphs with <code>3</code> and <code>4</code> neighbors respectively, centered at the blue node.</p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-descdrawio.png" style="width: 400px; height: 179px;" />
<p>The <strong>star sum</strong> is the sum of the values of all the nodes present in the star graph.</p>

<p>Given an integer <code>k</code>, return <em>the <strong>maximum star sum</strong> of a star graph containing <strong>at most</strong> </em><code>k</code><em> edges.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-example1drawio.png" style="width: 300px; height: 291px;" />
<pre>
<strong>Input:</strong> vals = [1,2,3,4,10,-10,-20], edges = [[0,1],[1,2],[1,3],[3,4],[3,5],[3,6]], k = 2
<strong>Output:</strong> 16
<strong>Explanation:</strong> The above diagram represents the input graph.
The star graph with the maximum star sum is denoted by blue. It is centered at 3 and includes its neighbors 1 and 4.
It can be shown it is not possible to get a star graph with a sum greater than 16.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> vals = [-5], edges = [], k = 0
<strong>Output:</strong> -5
<strong>Explanation:</strong> There is only one possible star graph, which is node 0 itself.
Hence, we return -5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == vals.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= vals[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= edges.length &lt;= min(n * (n - 1) / 2</code><code>, 10<sup>5</sup>)</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li><code>0 &lt;= k &lt;= n - 1</code></li>
</ul>


## Hints

1. A star graph doesn’t necessarily include all of its neighbors.
2. For each node, sort its neighbors in descending order and take k max valued neighbors.

## Solution

```rust
impl Solution { pub fn max_star_sum(black_vals: Vec<i32>, black_edges: Vec<Vec<i32>>, black_k: i32) -> i32 { let mut black_adj = vec![vec![]; black_vals.len()]; for black_e in black_edges { if black_vals[black_e[1] as usize] > 0 { black_adj[black_e[0] as usize].push(black_vals[black_e[1] as usize]); } if black_vals[black_e[0] as usize] > 0 { black_adj[black_e[1] as usize].push(black_vals[black_e[0] as usize]); } } let mut black_max = i32::MIN; for black_i in 0..black_vals.len() { black_adj[black_i].sort_unstable_by_key(|&black_v| -black_v); let black_sum: i32 = black_vals[black_i] + black_adj[black_i].iter().take(black_k as usize).sum::<i32>(); black_max = black_max.max(black_sum); } black_max } }
```