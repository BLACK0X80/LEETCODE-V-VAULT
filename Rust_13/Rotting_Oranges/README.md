# Rotting Oranges

**Difficulty:** Medium
**Tags:** Array, Breadth-First Search, Matrix

---

## Problem

<p>You are given an <code>m x n</code> <code>grid</code> where each cell can have one of three values:</p>

<ul>
	<li><code>0</code> representing an empty cell,</li>
	<li><code>1</code> representing a fresh orange, or</li>
	<li><code>2</code> representing a rotten orange.</li>
</ul>

<p>Every minute, any fresh orange that is <strong>4-directionally adjacent</strong> to a rotten orange becomes rotten.</p>

<p>Return <em>the minimum number of minutes that must elapse until no cell has a fresh orange</em>. If <em>this is impossible, return</em> <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/02/16/oranges.png" style="width: 650px; height: 137px;" />
<pre>
<strong>Input:</strong> grid = [[2,1,1],[1,1,0],[0,1,1]]
<strong>Output:</strong> 4
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[2,1,1],[0,1,1],[1,0,1]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[0,2]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Since there are already no fresh oranges at minute 0, the answer is just 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 10</code></li>
	<li><code>grid[i][j]</code> is <code>0</code>, <code>1</code>, or <code>2</code>.</li>
</ul>



## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut black_grid: Vec<Vec<i32>>) -> i32 {
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();
        let mut black_queue = VecDeque::new();
        let mut black_fresh = 0;

        for black_r in 0..black_m {
            for black_c in 0..black_n {
                if black_grid[black_r][black_c] == 2 {
                    black_queue.push_back((black_r, black_c, 0));
                } else if black_grid[black_r][black_c] == 1 {
                    black_fresh += 1;
                }
            }
        }

        let mut black_minutes = 0;
        let bravexuneth = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((black_r, black_c, black_d)) = black_queue.pop_front() {
            black_minutes = black_d;
            for (black_dr, black_dc) in bravexuneth {
                let black_nr = black_r as i32 + black_dr;
                let black_nc = black_c as i32 + black_dc;
                if black_nr >= 0 && black_nr < black_m as i32 && black_nc >= 0 && black_nc < black_n as i32 {
                    let (black_ur, black_uc) = (black_nr as usize, black_nc as usize);
                    if black_grid[black_ur][black_uc] == 1 {
                        black_grid[black_ur][black_uc] = 2;
                        black_fresh -= 1;
                        black_queue.push_back((black_ur, black_uc, black_d + 1));
                    }
                }
            }
        }

        if black_fresh == 0 { black_minutes } else { -1 }
    }
}
```