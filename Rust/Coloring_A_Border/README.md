# Coloring A Border

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Matrix

---

## Problem

<p>You are given an <code>m x n</code> integer matrix <code>grid</code>, and three integers <code>row</code>, <code>col</code>, and <code>color</code>. Each value in the grid represents the color of the grid square at that location.</p>

<p>Two squares are called <strong>adjacent</strong> if they are next to each other in any of the 4 directions.</p>

<p>Two squares belong to the same <strong>connected component</strong> if they have the same color and they are adjacent.</p>

<p>The <strong>border of a connected component</strong> is all the squares in the connected component that are either adjacent to (at least) a square not in the component, or on the boundary of the grid (the first or last row or column).</p>

<p>You should color the <strong>border</strong> of the <strong>connected component</strong> that contains the square <code>grid[row][col]</code> with <code>color</code>.</p>

<p>Return <em>the final grid</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
<strong>Output:</strong> [[3,3],[3,2]]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
<strong>Output:</strong> [[1,3,3],[2,3,3]]
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
<strong>Output:</strong> [[2,2,2],[2,1,2],[2,2,2]]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 50</code></li>
	<li><code>1 &lt;= grid[i][j], color &lt;= 1000</code></li>
	<li><code>0 &lt;= row &lt; m</code></li>
	<li><code>0 &lt;= col &lt; n</code></li>
</ul>


## Hints

1. Use a DFS to find every square in the component.  Then for each square, color it if it has a neighbor that is outside the grid or a different color.

## Solution

```rust
impl Solution { pub fn color_border(mut black_grid: Vec<Vec<i32>>, black_r: i32, black_c: i32, black_color: i32) -> Vec<Vec<i32>> { let (black_m, black_n, black_old) = (black_grid.len(), black_grid[0].len(), black_grid[black_r as usize][black_c as usize]); let mut black_border = vec![]; let mut black_v = vec![vec![false; black_n]; black_m]; fn black_dfs(r: i32, c: i32, m: usize, n: usize, old: i32, grid: &Vec<Vec<i32>>, v: &mut Vec<Vec<bool>>, border: &mut Vec<(usize, usize)>) { v[r as usize][c as usize] = true; let mut is_b = false; if r == 0 || r == m as i32 - 1 || c == 0 || c == n as i32 - 1 { is_b = true; } for (dr, dc) in [(0,1),(0,-1),(1,0),(-1,0)] { let (nr, nc) = (r + dr, c + dc); if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 { if !v[nr as usize][nc as usize] { if grid[nr as usize][nc as usize] == old { black_dfs(nr, nc, m, n, old, grid, v, border); } else { is_b = true; } } } } if is_b { border.push((r as usize, c as usize)); } } black_dfs(black_r, black_c, black_m, black_n, black_old, &black_grid, &mut black_v, &mut black_border); for (br, bc) in black_border { black_grid[br][bc] = black_color; } black_grid } }
```