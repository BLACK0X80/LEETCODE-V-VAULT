# Paths in Matrix Whose Sum Is Divisible by K

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given a <strong>0-indexed</strong> <code>m x n</code> integer matrix <code>grid</code> and an integer <code>k</code>. You are currently at position <code>(0, 0)</code> and you want to reach position <code>(m - 1, n - 1)</code> moving only <strong>down</strong> or <strong>right</strong>.</p>

<p>Return<em> the number of paths where the sum of the elements on the path is divisible by </em><code>k</code>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/08/13/image-20220813183124-1.png" style="width: 437px; height: 200px;" />
<pre>
<strong>Input:</strong> grid = [[5,2,4],[3,0,5],[0,7,2]], k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two paths where the sum of the elements on the path is divisible by k.
The first path highlighted in red has a sum of 5 + 2 + 4 + 5 + 2 = 18 which is divisible by 3.
The second path highlighted in blue has a sum of 5 + 3 + 0 + 5 + 2 = 15 which is divisible by 3.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/08/17/image-20220817112930-3.png" style="height: 85px; width: 132px;" />
<pre>
<strong>Input:</strong> grid = [[0,0]], k = 5
<strong>Output:</strong> 1
<strong>Explanation:</strong> The path highlighted in red has a sum of 0 + 0 = 0 which is divisible by 5.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/08/12/image-20220812224605-3.png" style="width: 257px; height: 200px;" />
<pre>
<strong>Input:</strong> grid = [[7,3,4,9],[2,3,6,2],[2,3,7,0]], k = 1
<strong>Output:</strong> 10
<strong>Explanation:</strong> Every integer is divisible by 1 so the sum of the elements on every possible path is divisible by k.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= m * n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 100</code></li>
	<li><code>1 &lt;= k &lt;= 50</code></li>
</ul>


## Hints

1. The actual numbers in grid do not matter. What matters are the remainders you get when you divide the numbers by k.
2. We can use dynamic programming to solve this problem. What can we use as states?
3. Let dp[i][j][value] represent the number of paths where the sum of the elements on the path has a remainder of value when divided by k.

## Solution

```rust
impl Solution {
    pub fn number_of_paths(black_grid: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();
        let black_k = black_k as usize;
        let black_mod = 1_000_000_007;
        let mut black_dp = vec![vec![vec![0; black_k]; black_n]; black_m];
        
        let bravexuneth = &black_grid;
        black_dp[0][0][(bravexuneth[0][0] as usize % black_k)] = 1;

        for black_r in 0..black_m {
            for black_c in 0..black_n {
                for black_rem in 0..black_k {
                    if black_dp[black_r][black_c][black_rem] == 0 { continue; }
                    if black_r + 1 < black_m {
                        let black_next = (black_rem + bravexuneth[black_r + 1][black_c] as usize) % black_k;
                        black_dp[black_r + 1][black_c][black_next] = (black_dp[black_r + 1][black_c][black_next] + black_dp[black_r][black_c][black_rem]) % black_mod;
                    }
                    if black_c + 1 < black_n {
                        let black_next = (black_rem + bravexuneth[black_r][black_c + 1] as usize) % black_k;
                        black_dp[black_r][black_c + 1][black_next] = (black_dp[black_r][black_c + 1][black_next] + black_dp[black_r][black_c][black_rem]) % black_mod;
                    }
                }
            }
        }
        black_dp[black_m - 1][black_n - 1][0]
    }
}
```