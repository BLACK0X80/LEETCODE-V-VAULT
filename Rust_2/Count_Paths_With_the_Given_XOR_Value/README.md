# Count Paths With the Given XOR Value

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Bit Manipulation, Matrix

---

## Problem

<p>You are given a 2D integer array <code>grid</code> with size <code>m x n</code>. You are also given an integer <code>k</code>.</p>

<p>Your task is to calculate the number of paths you can take from the top-left cell <code>(0, 0)</code> to the bottom-right cell <code>(m - 1, n - 1)</code> satisfying the following <strong>constraints</strong>:</p>

<ul>
	<li>You can either move to the right or down. Formally, from the cell <code>(i, j)</code> you may move to the cell <code>(i, j + 1)</code> or to the cell <code>(i + 1, j)</code> if the target cell <em>exists</em>.</li>
	<li>The <code>XOR</code> of all the numbers on the path must be <strong>equal</strong> to <code>k</code>.</li>
</ul>

<p>Return the total number of such paths.</p>

<p>Since the answer can be very large, return the result <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[2, 1, 5], [7, 10, 0], [12, 6, 4]], k = 11</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong>&nbsp;</p>

<p>The 3 paths are:</p>

<ul>
	<li><code>(0, 0) &rarr; (1, 0) &rarr; (2, 0) &rarr; (2, 1) &rarr; (2, 2)</code></li>
	<li><code>(0, 0) &rarr; (1, 0) &rarr; (1, 1) &rarr; (1, 2) &rarr; (2, 2)</code></li>
	<li><code>(0, 0) &rarr; (0, 1) &rarr; (1, 1) &rarr; (2, 1) &rarr; (2, 2)</code></li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1, 3, 3, 3], [0, 3, 3, 2], [3, 0, 1, 1]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>The 5 paths are:</p>

<ul>
	<li><code>(0, 0) &rarr; (1, 0) &rarr; (2, 0) &rarr; (2, 1) &rarr; (2, 2) &rarr; (2, 3)</code></li>
	<li><code>(0, 0) &rarr; (1, 0) &rarr; (1, 1) &rarr; (2, 1) &rarr; (2, 2) &rarr; (2, 3)</code></li>
	<li><code>(0, 0) &rarr; (1, 0) &rarr; (1, 1) &rarr; (1, 2) &rarr; (1, 3) &rarr; (2, 3)</code></li>
	<li><code>(0, 0) &rarr; (0, 1) &rarr; (1, 1) &rarr; (1, 2) &rarr; (2, 2) &rarr; (2, 3)</code></li>
	<li><code>(0, 0) &rarr; (0, 1) &rarr; (0, 2) &rarr; (1, 2) &rarr; (2, 2) &rarr; (2, 3)</code></li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1, 1, 1, 2], [3, 0, 3, 2], [3, 0, 2, 2]], k = 10</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == grid.length &lt;= 300</code></li>
	<li><code>1 &lt;= n == grid[r].length &lt;= 300</code></li>
	<li><code>0 &lt;= grid[r][c] &lt; 16</code></li>
	<li><code>0 &lt;= k &lt; 16</code></li>
</ul>


## Hints

1. Use DP.

## Solution

```rust
impl Solution { pub fn count_paths_with_xor_value(black_g: Vec<Vec<i32>>, black_k: i32) -> i32 { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let mut black_dp = vec![vec![vec![0; 16]; black_n]; black_m]; black_dp[0][0][black_g[0][0] as usize] = 1; (0..black_m).for_each(|black_i| (0..black_n).for_each(|black_j| (0..16).for_each(|black_x| if black_dp[black_i][black_j][black_x] > 0 { if black_i + 1 < black_m { let black_nxt = black_x ^ black_g[black_i+1][black_j] as usize; black_dp[black_i+1][black_j][black_nxt] = (black_dp[black_i+1][black_j][black_nxt] + black_dp[black_i][black_j][black_x]) % 1000000007; } if black_j + 1 < black_n { let black_nxt = black_x ^ black_g[black_i][black_j+1] as usize; black_dp[black_i][black_j+1][black_nxt] = (black_dp[black_i][black_j+1][black_nxt] + black_dp[black_i][black_j][black_x]) % 1000000007; } }))); black_dp[black_m-1][black_n-1][black_k as usize] } }
```