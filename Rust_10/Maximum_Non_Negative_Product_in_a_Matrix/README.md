# Maximum Non Negative Product in a Matrix

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given a <code>m x n</code> matrix <code>grid</code>. Initially, you are located at the top-left corner <code>(0, 0)</code>, and in each step, you can only <strong>move right or down</strong> in the matrix.</p>

<p>Among all possible paths starting from the top-left corner <code>(0, 0)</code> and ending in the bottom-right corner <code>(m - 1, n - 1)</code>, find the path with the <strong>maximum non-negative product</strong>. The product of a path is the product of all integers in the grid cells visited along the path.</p>

<p>Return the <em>maximum non-negative product <strong>modulo</strong> </em><code>10<sup>9</sup> + 7</code>. <em>If the maximum product is <strong>negative</strong>, return </em><code>-1</code>.</p>

<p>Notice that the modulo is performed after getting the maximum product.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/product1.jpg" style="width: 244px; height: 245px;" />
<pre>
<strong>Input:</strong> grid = [[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is not possible to get non-negative product in the path from (0, 0) to (2, 2), so return -1.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/product2.jpg" style="width: 244px; height: 245px;" />
<pre>
<strong>Input:</strong> grid = [[1,-2,1],[1,-2,1],[3,-4,1]]
<strong>Output:</strong> 8
<strong>Explanation:</strong> Maximum non-negative product is shown (1 * 1 * -2 * -4 * 1 = 8).
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/product3.jpg" style="width: 164px; height: 165px;" />
<pre>
<strong>Input:</strong> grid = [[1,3],[0,-4]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Maximum non-negative product is shown (1 * 0 * -4 = 0).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 15</code></li>
	<li><code>-4 &lt;= grid[i][j] &lt;= 4</code></li>
</ul>


## Hints

1. Use Dynamic programming. Keep the highest value and lowest value you can achieve up to a point.

## Solution

```rust
impl Solution { pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 { let (m, n) = (grid.len(), grid[0].len()); let mut dp: Vec<Vec<(i64, i64)>> = vec![vec![(0, 0); n]; m]; dp[0][0] = (grid[0][0] as i64, grid[0][0] as i64); for j in 1..n { let v = dp[0][j-1].0 * grid[0][j] as i64; dp[0][j] = (v, v); } for i in 1..m { let v = dp[i-1][0].0 * grid[i][0] as i64; dp[i][0] = (v, v); } for i in 1..m { for j in 1..n { let (v1, v2) = (grid[i][j] as i64 * dp[i-1][j].0, grid[i][j] as i64 * dp[i-1][j].1); let (v3, v4) = (grid[i][j] as i64 * dp[i][j-1].0, grid[i][j] as i64 * dp[i][j-1].1); let vals = [v1, v2, v3, v4]; dp[i][j] = (*vals.iter().max().unwrap(), *vals.iter().min().unwrap()); } } let res = dp[m-1][n-1].0; if res < 0 { -1 } else { (res % 1_000_000_007) as i32 } } }
```