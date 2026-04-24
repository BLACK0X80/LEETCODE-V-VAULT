# Check if There is a Valid Path in a Grid

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>You are given an <code>m x n</code> <code>grid</code>. Each cell of <code>grid</code> represents a street. The street of <code>grid[i][j]</code> can be:</p>

<ul>
	<li><code>1</code> which means a street connecting the left cell and the right cell.</li>
	<li><code>2</code> which means a street connecting the upper cell and the lower cell.</li>
	<li><code>3</code> which means a street connecting the left cell and the lower cell.</li>
	<li><code>4</code> which means a street connecting the right cell and the lower cell.</li>
	<li><code>5</code> which means a street connecting the left cell and the upper cell.</li>
	<li><code>6</code> which means a street connecting the right cell and the upper cell.</li>
</ul>
<img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/main.png" style="width: 450px; height: 708px;" />
<p>You will initially start at the street of the upper-left cell <code>(0, 0)</code>. A valid path in the grid is a path that starts from the upper left cell <code>(0, 0)</code> and ends at the bottom-right cell <code>(m - 1, n - 1)</code>. <strong>The path should only follow the streets</strong>.</p>

<p><strong>Notice</strong> that you are <strong>not allowed</strong> to change any street.</p>

<p>Return <code>true</code><em> if there is a valid path in the grid or </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/e1.png" style="width: 455px; height: 311px;" />
<pre>
<strong>Input:</strong> grid = [[2,4,3],[6,5,2]]
<strong>Output:</strong> true
<strong>Explanation:</strong> As shown you can start at cell (0, 0) and visit all the cells of the grid to reach (m - 1, n - 1).
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/e2.png" style="width: 455px; height: 293px;" />
<pre>
<strong>Input:</strong> grid = [[1,2,1],[1,2,1]]
<strong>Output:</strong> false
<strong>Explanation:</strong> As shown you the street at cell (0, 0) is not connected with any street of any other cell and you will get stuck at cell (0, 0)
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1,2]]
<strong>Output:</strong> false
<strong>Explanation:</strong> You will get stuck at cell (0, 1) and you cannot reach cell (0, 2).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 300</code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 6</code></li>
</ul>


## Hints

1. Start DFS from the node (0, 0) and follow the path till you stop.
2. When you reach a cell and cannot move anymore check that this cell is (m - 1, n - 1) or not.

## Solution

```rust
impl Solution { pub fn has_valid_path(black_g: Vec<Vec<i32>>) -> bool { let (black_r, black_c) = (black_g.len(), black_g[0].len()); let mut black_f: Vec<usize> = (0..black_r*black_c).collect(); fn black_find(mut i: usize, f: &mut Vec<usize>) -> usize { while f[i] != i { f[i] = f[f[i]]; i = f[i]; } i } let black_d = vec![vec![], vec![0, 1], vec![2, 3], vec![0, 3], vec![1, 3], vec![0, 2], vec![1, 2]]; for i in 0..black_r { for j in 0..black_c { let black_type = black_g[i][j] as usize; for &dir in &black_d[black_type] { let (ni, nj, nd) = match dir { 0 => (i as i32, j as i32 - 1, 1), 1 => (i as i32, j as i32 + 1, 0), 2 => (i as i32 - 1, j as i32, 3), _ => (i as i32 + 1, j as i32, 2) }; if ni >= 0 && ni < black_r as i32 && nj >= 0 && nj < black_c as i32 && black_d[black_g[ni as usize][nj as usize] as usize].contains(&nd) { let (r1, r2) = (black_find(i*black_c+j, &mut black_f), black_find(ni as usize * black_c + nj as usize, &mut black_f)); if r1 != r2 { black_f[r1] = r2; } } } } } black_find(0, &mut black_f) == black_find(black_r*black_c-1, &mut black_f) } }
```