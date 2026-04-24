# Shortest Bridge

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Matrix

---

## Problem

<p>You are given an <code>n x n</code> binary matrix <code>grid</code> where <code>1</code> represents land and <code>0</code> represents water.</p>

<p>An <strong>island</strong> is a 4-directionally connected group of <code>1</code>&#39;s not connected to any other <code>1</code>&#39;s. There are <strong>exactly two islands</strong> in <code>grid</code>.</p>

<p>You may change <code>0</code>&#39;s to <code>1</code>&#39;s to connect the two islands to form <strong>one island</strong>.</p>

<p>Return <em>the smallest number of </em><code>0</code><em>&#39;s you must flip to connect the two islands</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> grid = [[0,1],[1,0]]
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[0,1,0],[0,0,0],[0,0,1]]
<strong>Output:</strong> 2
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length == grid[i].length</code></li>
	<li><code>2 &lt;= n &lt;= 100</code></li>
	<li><code>grid[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
	<li>There are exactly two islands in <code>grid</code>.</li>
</ul>



## Solution

```rust
use std::collections::VecDeque;
impl Solution { pub fn shortest_bridge(mut black_grid: Vec<Vec<i32>>) -> i32 { let black_n = black_grid.len(); let mut black_q = VecDeque::new(); 'outer: for r in 0..black_n { for c in 0..black_n { if black_grid[r][c] == 1 { fn black_dfs(r: usize, c: usize, n: usize, g: &mut Vec<Vec<i32>>, q: &mut VecDeque<(usize, usize)>) { g[r][c] = 2; q.push_back((r, c)); for (dr, dc) in [(0,1),(0,-1),(1,0),(-1,0)] { let (nr, nc) = (r as i32 + dr, c as i32 + dc); if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 && g[nr as usize][nc as usize] == 1 { black_dfs(nr as usize, nc as usize, n, g, q); } } } black_dfs(r, c, black_n, &mut black_grid, &mut black_q); break 'outer; } } } let mut black_dist = 0; while !black_q.is_empty() { for _ in 0..black_q.len() { let (r, c) = black_q.pop_front().unwrap(); for (dr, dc) in [(0,1),(0,-1),(1,0),(-1,0)] { let (nr, nc) = (r as i32 + dr, c as i32 + dc); if nr >= 0 && nr < black_n as i32 && nc >= 0 && nc < black_n as i32 { let (unr, unc) = (nr as usize, nc as usize); if black_grid[unr][unc] == 1 { return black_dist; } if black_grid[unr][unc] == 0 { black_grid[unr][unc] = 2; black_q.push_back((unr, unc)); } } } } black_dist += 1; } 0 } }
```