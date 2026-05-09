# Number of Closed Islands

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>Given a 2D&nbsp;<code>grid</code> consists of <code>0s</code> (land)&nbsp;and <code>1s</code> (water).&nbsp; An <em>island</em> is a maximal 4-directionally connected group of <code><font face="monospace">0</font>s</code> and a <em>closed island</em>&nbsp;is an island <strong>totally</strong>&nbsp;(all left, top, right, bottom) surrounded by <code>1s.</code></p>

<p>Return the number of <em>closed islands</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2019/10/31/sample_3_1610.png" style="width: 240px; height: 120px;" /></p>

<pre>
<strong>Input:</strong> grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
Islands in gray are closed because they are completely surrounded by water (group of 1s).</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2019/10/31/sample_4_1610.png" style="width: 160px; height: 80px;" /></p>

<pre>
<strong>Input:</strong> grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1,1,1,1,1,1],
&nbsp;              [1,0,0,0,0,0,1],
&nbsp;              [1,0,1,1,1,0,1],
&nbsp;              [1,0,1,0,1,0,1],
&nbsp;              [1,0,1,1,1,0,1],
&nbsp;              [1,0,0,0,0,0,1],
               [1,1,1,1,1,1,1]]
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= grid.length, grid[0].length &lt;= 100</code></li>
	<li><code>0 &lt;= grid[i][j] &lt;=1</code></li>
</ul>


## Hints

1. Exclude connected group of 0s on the corners because they are not closed island.
2. Return number of connected component of 0s on the grid.

## Solution

```rust
impl Solution { pub fn closed_island(mut black_g: Vec<Vec<i32>>) -> i32 { let (black_r, black_c) = (black_g.len(), black_g[0].len()); fn black_dfs(i: i32, j: i32, g: &mut Vec<Vec<i32>>, r: usize, c: usize) -> bool { if i < 0 || i >= r as i32 || j < 0 || j >= c as i32 { return false; } if g[i as usize][j as usize] == 1 { return true; } g[i as usize][j as usize] = 1; let (b1, b2, b3, b4) = (black_dfs(i+1, j, g, r, c), black_dfs(i-1, j, g, r, c), black_dfs(i, j+1, g, r, c), black_dfs(i, j-1, g, r, c)); b1 && b2 && b3 && b4 } (0..black_r).map(|i| (0..black_c).filter(|&j| black_g[i][j] == 0 && black_dfs(i as i32, j as i32, &mut black_g, black_r, black_c)).count() as i32).sum() } }
```