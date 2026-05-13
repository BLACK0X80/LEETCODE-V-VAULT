# Count Islands With Total Value Divisible by K

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>You are given an <code>m x n</code> matrix <code>grid</code> and a positive integer <code>k</code>. An <strong>island</strong> is a group of <strong>positive</strong> integers (representing land) that are <strong>4-directionally</strong> connected (horizontally or vertically).</p>

<p>The <strong>total value</strong> of an island is the sum of the values of all cells in the island.</p>

<p>Return the number of islands with a total value <strong>divisible by</strong> <code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2025/03/06/example1griddrawio-1.png" style="width: 200px; height: 200px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0,2,1,0,0],[0,5,0,0,5],[0,0,1,0,0],[0,1,4,7,0],[0,2,0,0,8]], k = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid contains four islands. The islands highlighted in blue have a total value that is divisible by 5, while the islands highlighted in red do not.</p>
</div>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2025/03/06/example2griddrawio.png" style="width: 200px; height: 150px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[3,0,3,0], [0,3,0,3], [3,0,3,0]], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid contains six islands, each with a total value that is divisible by 3.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 1000</code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Use a BFS/DFS to find connected components.

## Solution

```rust
impl Solution { pub fn count_islands(black_g: Vec<Vec<i32>>, black_k: i32) -> i32 { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let mut black_v = vec![vec![false; black_n]; black_m]; let mut black_res = 0; let mut black_g = black_g; for r in 0..black_m { for c in 0..black_n { if black_g[r][c] > 0 && !black_v[r][c] { let mut black_s = 0i64; let mut black_stk = vec![(r, c)]; black_v[r][c] = true; while let Some((black_curr_r, black_curr_c)) = black_stk.pop() { black_s += black_g[black_curr_r][black_curr_c] as i64; for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] { let (nr, nc) = (black_curr_r as i32 + dr, black_curr_c as i32 + dc); if nr >= 0 && nr < black_m as i32 && nc >= 0 && nc < black_n as i32 { let (unr, unc) = (nr as usize, nc as usize); if black_g[unr][unc] > 0 && !black_v[unr][unc] { black_v[unr][unc] = true; black_stk.push((unr, unc)); } } } } if black_s % black_k as i64 == 0 { black_res += 1; } } } } black_res } }
```