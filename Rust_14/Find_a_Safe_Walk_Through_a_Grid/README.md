# Find a Safe Walk Through a Grid

**Difficulty:** Medium
**Tags:** Array, Breadth-First Search, Graph Theory, Heap (Priority Queue), Matrix, Shortest Path

---

## Problem

<p>You are given an <code>m x n</code> binary matrix <code>grid</code> and an integer <code>health</code>.</p>

<p>You start on the upper-left corner <code>(0, 0)</code> and would like to get to the lower-right corner <code>(m - 1, n - 1)</code>.</p>

<p>You can move up, down, left, or right from one cell to another adjacent cell as long as your health <em>remains</em> <strong>positive</strong>.</p>

<p>Cells <code>(i, j)</code> with <code>grid[i][j] = 1</code> are considered <strong>unsafe</strong> and reduce your health by 1.</p>

<p>Return <code>true</code> if you can reach the final cell with a health value of 1 or more, and <code>false</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]], health = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p>The final cell can be reached safely by walking along the gray cells below.</p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/08/04/3868_examples_1drawio.png" style="width: 301px; height: 121px;" /></div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0,1,1,0,0,0],[1,0,1,0,0,0],[0,1,1,1,0,1],[0,0,1,0,1,0]], health = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<p>A minimum of 4 health points is needed to reach the final cell safely.</p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/08/04/3868_examples_2drawio.png" style="width: 361px; height: 161px;" /></div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,1,1],[1,0,1],[1,1,1]], health = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p>The final cell can be reached safely by walking along the gray cells below.</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/04/3868_examples_3drawio.png" style="width: 181px; height: 121px;" /></p>

<p>Any path that does not go through the cell <code>(1, 1)</code> is unsafe since your health will drop to 0 when reaching the final cell.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 50</code></li>
	<li><code><font face="monospace">2 &lt;= m * n</font></code></li>
	<li><code>1 &lt;= health &lt;= m + n</code></li>
	<li><code>grid[i][j]</code> is either 0 or 1.</li>
</ul>


## Hints

1. Use 01 BFS.

## Solution

```rust
use std::collections::VecDeque; impl Solution { pub fn find_safe_walk(black_g: Vec<Vec<i32>>, black_h: i32) -> bool { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let mut black_d = vec![vec![i32::MAX; black_n]; black_m]; let mut black_q = VecDeque::from([(0, 0, black_g[0][0])]); black_d[0][0] = black_g[0][0]; while let Some((r, c, black_c)) = black_q.pop_front() { if black_c > black_d[r][c] { continue; } for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] { let (nr, nc) = (r as i32 + dr, c as i32 + dc); if nr >= 0 && nr < black_m as i32 && nc >= 0 && nc < black_n as i32 { let (unr, unc) = (nr as usize, nc as usize); let black_nc = black_c + black_g[unr][unc]; if black_nc < black_d[unr][unc] { black_d[unr][unc] = black_nc; if black_g[unr][unc] == 0 { black_q.push_front((unr, unc, black_nc)); } else { black_q.push_back((unr, unc, black_nc)); } } } } } black_h - black_d[black_m - 1][black_n - 1] >= 1 } }
```