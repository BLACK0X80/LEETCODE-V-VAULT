# Multi Source Flood Fill

**Difficulty:** Medium
**Tags:** Array, Breadth-First Search, Matrix

---

## Problem

<p>You are given two integers <code>n</code> and <code>m</code> representing the number of rows and columns of a grid, respectively.</p>

<p>You are also given a 2D integer array <code>sources</code>, where <code>sources[i] = [r<sub>i</sub>, c<sub>i</sub>, color<sub>​​​​​​​i</sub>]</code> indicates that the cell <code>(r<sub>i</sub>, c<sub>i</sub>)</code> is initially colored with <code>color<sub>i</sub></code>. All other cells are initially uncolored and represented as 0.</p>

<p>At each time step, every currently colored cell spreads its color to all adjacent <strong>uncolored</strong> cells in the four directions: up, down, left, and right. All spreads happen simultaneously.</p>

<p>If <strong>multiple</strong> colors reach the same uncolored cell at the same time step, the cell takes the color with the <strong>maximum</strong> value.</p>

<p>The process continues until no more cells can be colored.</p>

<p>Return a 2D integer array representing the final state of the grid, where each cell contains its final color.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, m = 3, sources = [[0,0,1],[2,2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[1,1,2],[1,2,2],[2,2,2]]</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid at each time step is as follows:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2026/03/29/g50new.png" style="width: 500px; height: 174px;" />​​​​​​​</p>

<p>At time step 2, cells <code>(0, 2)</code>, <code>(1, 1)</code>, and <code>(2, 0)</code> are reached by both colors, so they are assigned color 2 as it has the maximum value among them.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, m = 3, sources = [[0,1,3],[1,1,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[3,3,3],[5,5,5],[5,5,5]]</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid at each time step is as follows:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2026/03/29/g51new.png" style="width: 500px; height: 177px;" /></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2, m = 2, sources = [[1,1,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[5,5],[5,5]]</span></p>

<p><strong>Explanation:</strong></p>

<p>The grid at each time step is as follows:</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2026/03/29/g52new.png" style="width: 500px; height: 150px;" />​​​​​​​</p>

<p>Since there is only one source, all cells are assigned the same color.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n, m &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= n * m &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= sources.length &lt;= n * m</code></li>
	<li><code>sources[i] = [r<sub>i</sub>, c<sub>i</sub>, color<sub>i</sub>]</code></li>
	<li><code>0 &lt;= r<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>0 &lt;= c<sub>i</sub> &lt;= m - 1</code></li>
	<li><code>1 &lt;= color<sub>i</sub> &lt;= 10<sup>6</sup>​​​​​​​</code></li>
	<li>All <code>(r<sub>i</sub>, c<sub>i</sub>​​​​​​​)</code> in <code>sources</code> are distinct.</li>
</ul>


## Hints

1. Multi-source BFS
2. Initialize a queue with all colored cells
3. Spread colors level by level to adjacent cells in <code>4</code> directions
4. If multiple colors reach the same cell at the same time, assign the maximum color value

## Solution

```rust
use std::collections::VecDeque; impl Solution { pub fn color_grid(black_n: i32, black_m: i32, black_s: Vec<Vec<i32>>) -> Vec<Vec<i32>> { let (black_n, black_m) = (black_n as usize, black_m as usize); let (mut black_res, mut black_t, mut black_q) = (vec![vec![0; black_m]; black_n], vec![vec![i32::MAX; black_m]; black_n], VecDeque::new()); for black_src in black_s { let (r, c, black_v) = (black_src[0] as usize, black_src[1] as usize, black_src[2]); black_res[r][c] = black_v; black_t[r][c] = 0; black_q.push_back((r, c)); } while let Some((r, c)) = black_q.pop_front() { for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] { let (nr, nc) = (r as i32 + dr, c as i32 + dc); if nr >= 0 && nr < black_n as i32 && nc >= 0 && nc < black_m as i32 { let (nr, nc, black_nt) = (nr as usize, nc as usize, black_t[r][c] + 1); if black_nt < black_t[nr][nc] { black_t[nr][nc] = black_nt; black_res[nr][nc] = black_res[r][c]; black_q.push_back((nr, nc)); } else if black_nt == black_t[nr][nc] { black_res[nr][nc] = black_res[nr][nc].max(black_res[r][c]); } } } } black_res } }
```