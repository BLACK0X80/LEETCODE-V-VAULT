# Out of Boundary Paths

**Difficulty:** Medium
**Tags:** Dynamic Programming

---

## Problem

<p>There is an <code>m x n</code> grid with a ball. The ball is initially at the position <code>[startRow, startColumn]</code>. You are allowed to move the ball to one of the four adjacent cells in the grid (possibly out of the grid crossing the grid boundary). You can apply <strong>at most</strong> <code>maxMove</code> moves to the ball.</p>

<p>Given the five integers <code>m</code>, <code>n</code>, <code>maxMove</code>, <code>startRow</code>, <code>startColumn</code>, return the number of paths to move the ball out of the grid boundary. Since the answer can be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/out_of_boundary_paths_1.png" style="width: 500px; height: 296px;" />
<pre>
<strong>Input:</strong> m = 2, n = 2, maxMove = 2, startRow = 0, startColumn = 0
<strong>Output:</strong> 6
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/out_of_boundary_paths_2.png" style="width: 500px; height: 293px;" />
<pre>
<strong>Input:</strong> m = 1, n = 3, maxMove = 3, startRow = 0, startColumn = 1
<strong>Output:</strong> 12
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m, n &lt;= 50</code></li>
	<li><code>0 &lt;= maxMove &lt;= 50</code></li>
	<li><code>0 &lt;= startRow &lt; m</code></li>
	<li><code>0 &lt;= startColumn &lt; n</code></li>
</ul>


## Hints

1. Is traversing every path feasible? There are many possible paths for a small matrix. Try to optimize it.
2. Can we use some space to store the number of paths and update them after every move?
3. One obvious thing: the ball will go out of the boundary only by crossing it. Also, there is only one possible way the ball can go out of the boundary from the boundary cell except for corner cells. From the corner cell, the ball can go out in two different ways.

Can you use this thing to solve the problem?

## Solution

```rust
impl Solution { pub fn find_paths(m: i32, n: i32, max_move: i32, r: i32, c: i32) -> i32 { let mut black_dp = vec![vec![0; n as usize]; m as usize]; let black_mod = 1_000_000_007; for k in 1..=max_move as usize { let mut black_next = vec![vec![0; n as usize]; m as usize]; for i in 0..m as usize { for j in 0..n as usize { for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] { let (ni, nj) = (i as i32 + di, j as i32 + dj); if ni < 0 || ni >= m || nj < 0 || nj >= n { black_next[i][j] = (black_next[i][j] + 1) % black_mod; } else { black_next[i][j] = (black_next[i][j] + black_dp[ni as usize][nj as usize]) % black_mod; } } } } black_dp = black_next; } black_dp[r as usize][c as usize] } }
```