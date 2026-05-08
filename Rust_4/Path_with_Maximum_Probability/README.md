# Path with Maximum Probability

**Difficulty:** Medium
**Tags:** Array, Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>You are given an undirected weighted graph of&nbsp;<code>n</code>&nbsp;nodes (0-indexed), represented by an edge list where&nbsp;<code>edges[i] = [a, b]</code>&nbsp;is an undirected edge connecting the nodes&nbsp;<code>a</code>&nbsp;and&nbsp;<code>b</code>&nbsp;with a probability of success of traversing that edge&nbsp;<code>succProb[i]</code>.</p>

<p>Given two nodes&nbsp;<code>start</code>&nbsp;and&nbsp;<code>end</code>, find the path with the maximum probability of success to go from&nbsp;<code>start</code>&nbsp;to&nbsp;<code>end</code>&nbsp;and return its success probability.</p>

<p>If there is no path from&nbsp;<code>start</code>&nbsp;to&nbsp;<code>end</code>, <strong>return&nbsp;0</strong>. Your answer will be accepted if it differs from the correct answer by at most <strong>1e-5</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex1.png" style="width: 187px; height: 186px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.2], start = 0, end = 2
<strong>Output:</strong> 0.25000
<strong>Explanation:</strong>&nbsp;There are two paths from start to end, one having a probability of success = 0.2 and the other has 0.5 * 0.5 = 0.25.
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex2.png" style="width: 189px; height: 186px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[1,2],[0,2]], succProb = [0.5,0.5,0.3], start = 0, end = 2
<strong>Output:</strong> 0.30000
</pre>

<p><strong class="example">Example 3:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2019/09/20/1558_ex3.png" style="width: 215px; height: 191px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 3, edges = [[0,1]], succProb = [0.5], start = 0, end = 2
<strong>Output:</strong> 0.00000
<strong>Explanation:</strong>&nbsp;There is no path between 0 and 2.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10^4</code></li>
	<li><code>0 &lt;= start, end &lt; n</code></li>
	<li><code>start != end</code></li>
	<li><code>0 &lt;= a, b &lt; n</code></li>
	<li><code>a != b</code></li>
	<li><code>0 &lt;= succProb.length == edges.length &lt;= 2*10^4</code></li>
	<li><code>0 &lt;= succProb[i] &lt;= 1</code></li>
	<li>There is at most one edge between every two nodes.</li>
</ul>


## Hints

1. Multiplying probabilities will result in precision errors.
2. Take log probabilities to sum up numbers instead of multiplying them.
3. Use Dijkstra's algorithm to find the minimum path between the two nodes after negating all costs.

## Solution

```rust
impl Solution { pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 { let mut black_adj = vec![vec![]; n as usize]; for (i, e) in edges.iter().enumerate() { black_adj[e[0] as usize].push((e[1] as usize, succ_prob[i])); black_adj[e[1] as usize].push((e[0] as usize, succ_prob[i])); } let (mut black_probs, mut black_pq) = (vec![0.0; n as usize], std::collections::BinaryHeap::new()); black_probs[start as usize] = 1.0; black_pq.push(( (1.0f64).to_bits(), start as usize)); while let Some((black_p_bits, black_u)) = black_pq.pop() { let black_p = f64::from_bits(black_p_bits); if black_u == end as usize { return black_p; } if black_p < black_probs[black_u] { continue; } for &(black_v, black_pr) in &black_adj[black_u] { if black_probs[black_u] * black_pr > black_probs[black_v] { black_probs[black_v] = black_probs[black_u] * black_pr; black_pq.push((black_probs[black_v].to_bits(), black_v)); } } } 0.0 } }
```