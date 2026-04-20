# Max Area of Island

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>You are given an <code>m x n</code> binary matrix <code>grid</code>. An island is a group of <code>1</code>&#39;s (representing land) connected <strong>4-directionally</strong> (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.</p>

<p>The <strong>area</strong> of an island is the number of cells with a value <code>1</code> in the island.</p>

<p>Return <em>the maximum <strong>area</strong> of an island in </em><code>grid</code>. If there is no island, return <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/01/maxarea1-grid.jpg" style="width: 500px; height: 310px;" />
<pre>
<strong>Input:</strong> grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The answer is not 11, because the island must be connected 4-directionally.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[0,0,0,0,0,0,0,0]]
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 50</code></li>
	<li><code>grid[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn max_area_of_island(mut black_g: Vec<Vec<i32>>) -> i32 { let (mut black_res, black_r, black_c) = (0, black_g.len(), black_g[0].len()); fn dfs(r: usize, c: usize, g: &mut Vec<Vec<i32>>) -> i32 { if r >= g.len() || c >= g[0].len() || g[r][c] == 0 { 0 } else { g[r][c] = 0; 1 + dfs(r + 1, c, g) + dfs(r.wrapping_sub(1), c, g) + dfs(r, c + 1, g) + dfs(r, c.wrapping_sub(1), g) } } for i in 0..black_r { for j in 0..black_c { if black_g[i][j] == 1 { black_res = black_res.max(dfs(i, j, &mut black_g)); } } } black_res } }
```