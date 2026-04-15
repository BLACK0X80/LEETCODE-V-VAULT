# Build Array Where You Can Find The Maximum Exactly K Comparisons

**Difficulty:** Hard
**Tags:** Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given three integers <code>n</code>, <code>m</code> and <code>k</code>. Consider the following algorithm to find the maximum element of an array of positive integers:</p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/e.png" style="width: 424px; height: 372px;" />
<p>You should build the array arr which has the following properties:</p>

<ul>
	<li><code>arr</code> has exactly <code>n</code> integers.</li>
	<li><code>1 &lt;= arr[i] &lt;= m</code> where <code>(0 &lt;= i &lt; n)</code>.</li>
	<li>After applying the mentioned algorithm to <code>arr</code>, the value <code>search_cost</code> is equal to <code>k</code>.</li>
</ul>

<p>Return <em>the number of ways</em> to build the array <code>arr</code> under the mentioned conditions. As the answer may grow large, the answer <strong>must be</strong> computed modulo <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2, m = 3, k = 1
<strong>Output:</strong> 6
<strong>Explanation:</strong> The possible arrays are [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 5, m = 2, k = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no possible arrays that satisfy the mentioned conditions.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 9, m = 1, k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only possible array is [1, 1, 1, 1, 1, 1, 1, 1, 1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 50</code></li>
	<li><code>1 &lt;= m &lt;= 100</code></li>
	<li><code>0 &lt;= k &lt;= n</code></li>
</ul>


## Hints

1. Use dynamic programming approach. Build dp table where dp[a][b][c] is the number of ways you can start building the array starting from index a where the search_cost = c and the maximum used integer was b.
2. Recursively, solve the small sub-problems first. Optimize your answer by stopping the search if you exceeded k changes.

## Solution

```rust
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let (black_n, black_m, black_k) = (n as usize, m as usize, k as usize);
        if black_k == 0 { return 0; }
        let mut black_dp = vec![vec![vec![0i64; black_m + 1]; black_k + 1]; black_n + 1];
        for black_v in 1..=black_m { black_dp[1][1][black_v] = 1; }
        for black_i in 2..=black_n {
            for black_j in 1..=black_k {
                let mut black_pre_sum = 0i64;
                for black_v in 1..=black_m {
                    black_dp[black_i][black_j][black_v] = (black_dp[black_i - 1][black_j][black_v] * black_v as i64) % black_mod;
                    black_dp[black_i][black_j][black_v] = (black_dp[black_i][black_j][black_v] + black_pre_sum) % black_mod;
                    black_pre_sum = (black_pre_sum + black_dp[black_i - 1][black_j - 1][black_v]) % black_mod;
                }
            }
        }
        let mut black_res = 0;
        for black_v in 1..=black_m {
            black_res = (black_res + black_dp[black_n][black_k][black_v]) % black_mod;
        }
        black_res as i32
    }
}
```