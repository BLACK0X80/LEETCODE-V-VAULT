# Longest Increasing Path in a Matrix

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Depth-First Search, Breadth-First Search, Graph Theory, Topological Sort, Memoization, Matrix

---

## Problem

<p>Given an <code>m x n</code> integers <code>matrix</code>, return <em>the length of the longest increasing path in </em><code>matrix</code>.</p>

<p>From each cell, you can either move in four directions: left, right, up, or down. You <strong>may not</strong> move <strong>diagonally</strong> or move <strong>outside the boundary</strong> (i.e., wrap-around is not allowed).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/05/grid1.jpg" style="width: 242px; height: 242px;" />
<pre>
<strong>Input:</strong> matrix = [[9,9,4],[6,6,8],[2,1,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest increasing path is <code>[1, 2, 6, 9]</code>.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/27/tmp-grid.jpg" style="width: 253px; height: 253px;" />
<pre>
<strong>Input:</strong> matrix = [[3,4,5],[3,2,6],[2,2,1]]
<strong>Output:</strong> 4
<strong>Explanation: </strong>The longest increasing path is <code>[3, 4, 5, 6]</code>. Moving diagonally is not allowed.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[1]]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == matrix.length</code></li>
	<li><code>n == matrix[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 200</code></li>
	<li><code>0 &lt;= matrix[i][j] &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut memo = vec![vec![0i32; n]; m];
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                result = result.max(dfs(&matrix, &mut memo, i, j, m, n));
            }
        }
        result
    }
}

fn dfs(matrix: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, r: usize, c: usize, m: usize, n: usize) -> i32 {
    if memo[r][c] != 0 { return memo[r][c]; }
    let dirs = [(0i32,1),(0,-1),(1,0),(-1,0)];
    let mut best = 1;
    for (dr, dc) in dirs {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if matrix[nr][nc] > matrix[r][c] {
                best = best.max(1 + dfs(matrix, memo, nr, nc, m, n));
            }
        }
    }
    memo[r][c] = best;
    best
}
```