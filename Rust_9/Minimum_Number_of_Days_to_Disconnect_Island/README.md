# Minimum Number of Days to Disconnect Island

**Difficulty:** Hard
**Tags:** Array, Depth-First Search, Breadth-First Search, Matrix, Strongly Connected Component

---

## Problem

<p>You are given an <code>m x n</code> binary grid <code>grid</code> where <code>1</code> represents land and <code>0</code> represents water. An <strong>island</strong> is a maximal <strong>4-directionally</strong> (horizontal or vertical) connected group of <code>1</code>&#39;s.</p>

<p>The grid is said to be <strong>connected</strong> if we have <strong>exactly one island</strong>, otherwise is said <strong>disconnected</strong>.</p>

<p>In one day, we are allowed to change <strong>any </strong>single land cell <code>(1)</code> into a water cell <code>(0)</code>.</p>

<p>Return <em>the minimum number of days to disconnect the grid</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/24/land1.jpg" style="width: 500px; height: 169px;" />
<pre>
<strong>Input:</strong> grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]

<strong>Output:</strong> 2
<strong>Explanation:</strong> We need at least 2 days to get a disconnected grid.
Change land grid[1][1] and grid[0][2] to water and get 2 disconnected island.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/24/land2.jpg" style="width: 404px; height: 85px;" />
<pre>
<strong>Input:</strong> grid = [[1,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Grid of full water is also disconnected ([[1,1]] -&gt; [[0,0]]), 0 islands.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 30</code></li>
	<li><code>grid[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. Return 0 if the grid is already disconnected.
2. Return 1 if changing a single land to water disconnect the island.
3. Otherwise return 2.
4. We can disconnect the grid within at most 2 days.

## Solution

```rust
impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let black_count = |g: &Vec<Vec<i32>>| -> i32 {
            let mut black_vis = vec![vec![false; n]; m];
            let mut black_cnt = 0;
            for i in 0..m { for j in 0..n {
                if g[i][j] == 1 && !black_vis[i][j] {
                    black_cnt += 1;
                    let mut black_q = vec![(i, j)];
                    black_vis[i][j] = true;
                    while let Some((r, c)) = black_q.pop() {
                        for (dr, dc) in [(!0,0),(1,0),(0,!0usize),(0,1usize)] {
                            let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                            if nr < m && nc < n && !black_vis[nr][nc] && g[nr][nc] == 1 {
                                black_vis[nr][nc] = true;
                                black_q.push((nr, nc));
                            }
                        }
                    }
                }
            }}
            black_cnt
        };

        if black_count(&grid) != 1 { return 0; }

        for i in 0..m { for j in 0..n {
            if grid[i][j] == 1 {
                let mut black_g = grid.clone();
                black_g[i][j] = 0;
                if black_count(&black_g) != 1 { return 1; }
            }
        }}
        2
    }
}
```