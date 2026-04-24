# Shortest Path in Binary Matrix

**Difficulty:** Medium
**Tags:** Array, Breadth-First Search, Matrix

---

## Problem

<p>Given an <code>n x n</code> binary matrix <code>grid</code>, return <em>the length of the shortest <strong>clear path</strong> in the matrix</em>. If there is no clear path, return <code>-1</code>.</p>

<p>A <strong>clear path</strong> in a binary matrix is a path from the <strong>top-left</strong> cell (i.e., <code>(0, 0)</code>) to the <strong>bottom-right</strong> cell (i.e., <code>(n - 1, n - 1)</code>) such that:</p>

<ul>
	<li>All the visited cells of the path are <code>0</code>.</li>
	<li>All the adjacent cells of the path are <strong>8-directionally</strong> connected (i.e., they are different and they share an edge or a corner).</li>
</ul>

<p>The <strong>length of a clear path</strong> is the number of visited cells of this path.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/example1_1.png" style="width: 500px; height: 234px;" />
<pre>
<strong>Input:</strong> grid = [[0,1],[1,0]]
<strong>Output:</strong> 2
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/example2_1.png" style="height: 216px; width: 500px;" />
<pre>
<strong>Input:</strong> grid = [[0,0,0],[1,1,0],[1,1,0]]
<strong>Output:</strong> 4
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,0,0],[1,1,0],[1,1,0]]
<strong>Output:</strong> -1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= n &lt;= 100</code></li>
	<li><code>grid[i][j] is 0 or 1</code></li>
</ul>


## Hints

1. Do a breadth first search to find the shortest path.

## Solution

```rust
use std::collections::VecDeque;
impl Solution { pub fn shortest_path_binary_matrix(black_grid: Vec<Vec<i32>>) -> i32 { let black_n = black_grid.len(); if black_grid[0][0] == 1 || black_grid[black_n-1][black_n-1] == 1 { return -1; } let mut black_q = VecDeque::new(); black_q.push_back((0, 0, 1)); let mut black_v = vec![vec![false; black_n]; black_n]; black_v[0][0] = true; while let Some((black_r, black_c, black_d)) = black_q.pop_front() { if black_r == black_n - 1 && black_c == black_n - 1 { return black_d; } for black_dr in -1..=1 { for black_dc in -1..=1 { let (black_nr, black_nc) = (black_r as i32 + black_dr, black_c as i32 + black_dc); if black_nr >= 0 && black_nr < black_n as i32 && black_nc >= 0 && black_nc < black_n as i32 { let (black_unr, black_unc) = (black_nr as usize, black_nc as usize); if black_grid[black_unr][black_unc] == 0 && !black_v[black_unr][black_unc] { black_v[black_unr][black_unc] = true; black_q.push_back((black_unr, black_unc, black_d + 1)); } } } } } -1 } }
```