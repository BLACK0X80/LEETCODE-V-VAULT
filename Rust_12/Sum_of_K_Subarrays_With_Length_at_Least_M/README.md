# Sum of K Subarrays With Length at Least M

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> and two integers, <code>k</code> and <code>m</code>.</p>

<p>Return the <strong>maximum</strong> sum of <code>k</code> non-overlapping <span data-keyword="subarray">subarrays</span> of <code>nums</code>, where each subarray has a length of <strong>at least</strong> <code>m</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,-1,3,3,4], k = 2, m = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">13</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal choice is:</p>

<ul>
	<li>Subarray <code>nums[3..5]</code> with sum <code>3 + 3 + 4 = 10</code> (length is <code>3 &gt;= m</code>).</li>
	<li>Subarray <code>nums[0..1]</code> with sum <code>1 + 2 = 3</code> (length is <code>2 &gt;= m</code>).</li>
</ul>

<p>The total sum is <code>10 + 3 = 13</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [-10,3,-1,-2], k = 4, m = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">-10</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal choice is choosing each element as a subarray. The output is <code>(-10) + 3 + (-1) + (-2) = -10</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2000</code></li>
	<li><code>-10<sup>4</sup> &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= k &lt;= floor(nums.length / m)</code></li>
	<li><code>1 &lt;= m &lt;= 3</code></li>
</ul>


## Hints

1. Dynamic Programming
2. Prefix Sum
3. Let <code>dp[i][j]</code> be the maximum sum with <code>i</code> subarrays for the first <code>j</code> elements

## Solution

```rust
impl Solution { pub fn max_sum(black_nums: Vec<i32>, black_k: i32, black_m: i32) -> i32 { let (black_n, black_k, black_m) = (black_nums.len(), black_k as usize, black_m as usize); let mut black_pre = vec![0; black_n + 1]; for black_i in 0..black_n { black_pre[black_i + 1] = black_pre[black_i] + black_nums[black_i]; } let mut black_dp = vec![vec![-1000000000; black_k + 1]; black_n + 1]; for black_i in 0..=black_n { black_dp[black_i][0] = 0; } for black_j in 1..=black_k { let mut black_best = -1000000000; for black_i in black_m * black_j..=black_n { black_best = black_best.max(black_dp[black_i - black_m][black_j - 1] - black_pre[black_i - black_m]); black_dp[black_i][black_j] = black_dp[black_i - 1][black_j].max(black_best + black_pre[black_i]); } } black_dp[black_n][black_k] } }
```