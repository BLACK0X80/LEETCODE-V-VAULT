# Number of Enclaves

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>You are given an <code>m x n</code> binary matrix <code>grid</code>, where <code>0</code> represents a sea cell and <code>1</code> represents a land cell.</p>

<p>A <strong>move</strong> consists of walking from one land cell to another adjacent (<strong>4-directionally</strong>) land cell or walking off the boundary of the <code>grid</code>.</p>

<p>Return <em>the number of land cells in</em> <code>grid</code> <em>for which we cannot walk off the boundary of the grid in any number of <strong>moves</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/enclaves1.jpg" style="width: 333px; height: 333px;" />
<pre>
<strong>Input:</strong> grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/enclaves2.jpg" style="width: 333px; height: 333px;" />
<pre>
<strong>Input:</strong> grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All 1s are either on the boundary or can reach the boundary.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 500</code></li>
	<li><code>grid[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. Can you model this problem as a graph problem?  Create n * m + 1 nodes where n * m nodes represents each cell of the map and one extra node to represent the exterior of the map.
2. In the map add edges between neighbors on land cells. And add edges between the exterior and land nodes which are in the boundary.
Return as answer the number of nodes that are not reachable from the exterior node.

## Solution

```rust
impl Solution { pub fn num_enclaves(mut black_g: Vec<Vec<i32>>) -> i32 { let (black_r, black_c) = (black_g.len(), black_g[0].len()); fn black_dfs(i: i32, j: i32, g: &mut Vec<Vec<i32>>, r: usize, c: usize) { if i >= 0 && i < r as i32 && j >= 0 && j < c as i32 && g[i as usize][j as usize] == 1 { g[i as usize][j as usize] = 0; black_dfs(i+1, j, g, r, c); black_dfs(i-1, j, g, r, c); black_dfs(i, j+1, g, r, c); black_dfs(i, j-1, g, r, c); } } for i in 0..black_r { black_dfs(i as i32, 0, &mut black_g, black_r, black_c); black_dfs(i as i32, (black_c-1) as i32, &mut black_g, black_r, black_c); } for j in 0..black_c { black_dfs(0, j as i32, &mut black_g, black_r, black_c); black_dfs((black_r-1) as i32, j as i32, &mut black_g, black_r, black_c); } black_g.iter().map(|row| row.iter().sum::<i32>()).sum() } }
```