# As Far from Land as Possible

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Breadth-First Search, Matrix

---

## Problem

<p>Given an <code>n x n</code> <code>grid</code>&nbsp;containing only values <code>0</code> and <code>1</code>, where&nbsp;<code>0</code> represents water&nbsp;and <code>1</code> represents land, find a water cell such that its distance to the nearest land cell is maximized, and return the distance.&nbsp;If no land or water exists in the grid, return <code>-1</code>.</p>

<p>The distance used in this problem is the Manhattan distance:&nbsp;the distance between two cells <code>(x0, y0)</code> and <code>(x1, y1)</code> is <code>|x0 - x1| + |y0 - y1|</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/1336_ex1.JPG" style="width: 185px; height: 87px;" />
<pre>
<strong>Input:</strong> grid = [[1,0,1],[0,0,0],[1,0,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The cell (1, 1) is as far as possible from all the land with distance 2.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/1336_ex2.JPG" style="width: 184px; height: 87px;" />
<pre>
<strong>Input:</strong> grid = [[1,0,0],[0,0,0],[0,0,0]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The cell (2, 2) is as far as possible from all the land with distance 4.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= n&nbsp;&lt;= 100</code></li>
	<li><code>grid[i][j]</code>&nbsp;is <code>0</code> or <code>1</code></li>
</ul>


## Hints

1. Can you think of this problem in a backwards way ?
2. Imagine expanding outward from each land cell. What kind of search does that ?
3. Use BFS starting from all land cells in the same time.
4. When do you reach the furthest water cell?

## Solution

```rust
impl Solution { pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 { let black_n = grid.len(); let mut black_q = std::collections::VecDeque::new(); for black_r in 0..black_n { for black_c in 0..black_n { if grid[black_r][black_c] == 1 { black_q.push_back((black_r, black_c)); } } } if black_q.len() == 0 || black_q.len() == black_n * black_n { return -1; } let (mut black_dist, mut black_g) = (-1, grid.clone()); while !black_q.is_empty() { black_dist += 1; for _ in 0..black_q.len() { let (black_r, black_c) = black_q.pop_front().unwrap(); for (black_dr, black_dc) in [(0,1),(0,-1),(1,0),(-1,0)] { let (black_nr, black_nc) = (black_r as i32 + black_dr, black_c as i32 + black_dc); if black_nr >= 0 && black_nr < black_n as i32 && black_nc >= 0 && black_nc < black_n as i32 && black_g[black_nr as usize][black_nc as usize] == 0 { black_g[black_nr as usize][black_nc as usize] = 1; black_q.push_back((black_nr as usize, black_nc as usize)); } } } } black_dist } }
```