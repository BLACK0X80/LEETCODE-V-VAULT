# Special Permutations

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Bit Manipulation, Bitmask

---

## Problem

<p>You are given a&nbsp;<strong>0-indexed</strong>&nbsp;integer array&nbsp;<code>nums</code>&nbsp;containing&nbsp;<code>n</code>&nbsp;<strong>distinct</strong> positive integers. A permutation of&nbsp;<code>nums</code>&nbsp;is called special if:</p>

<ul>
	<li>For all indexes&nbsp;<code>0 &lt;= i &lt; n - 1</code>, either&nbsp;<code>nums[i] % nums[i+1] == 0</code>&nbsp;or&nbsp;<code>nums[i+1] % nums[i] == 0</code>.</li>
</ul>

<p>Return&nbsp;<em>the total number of special permutations.&nbsp;</em>As the answer could be large, return it&nbsp;<strong>modulo&nbsp;</strong><code>10<sup>9&nbsp;</sup>+ 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,3,6]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [3,6,2] and [2,6,3] are the two special permutations of nums.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,4,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [3,1,4] and [4,1,3] are the two special permutations of nums.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 14</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Can we solve this problem using DP with bit masking?
2. You just need two states in DP which are last_ind in the permutation and the mask of numbers already used.

## Solution

```rust
impl Solution { pub fn special_perm(black_v: Vec<i32>) -> i32 { let black_n = black_v.len(); let mut black_dp = vec![vec![-1i64; black_n]; 1 << black_n]; fn black_dfs(black_m: usize, black_p: usize, black_n: usize, black_v: &Vec<i32>, black_dp: &mut Vec<Vec<i64>>) -> i64 { if black_m == (1 << black_n) - 1 { return 1; } if black_dp[black_m][black_p] != -1 { return black_dp[black_m][black_p]; } let mut black_res = 0; for black_i in 0..black_n { if (black_m & (1 << black_i)) == 0 && (black_v[black_p] % black_v[black_i] == 0 || black_v[black_i] % black_v[black_p] == 0) { black_res = (black_res + black_dfs(black_m | (1 << black_i), black_i, black_n, black_v, black_dp)) % 1_000_000_007; } } black_dp[black_m][black_p] = black_res; black_res } let mut black_total = 0; for black_i in 0..black_n { black_total = (black_total + black_dfs(1 << black_i, black_i, black_n, &black_v, &mut black_dp)) % 1_000_000_007; } black_total as i32 } }
```