# Largest Divisible Subset

**Difficulty:** Medium
**Tags:** Array, Math, Dynamic Programming, Sorting

---

## Problem

<p>Given a set of <strong>distinct</strong> positive integers <code>nums</code>, return the largest subset <code>answer</code> such that every pair <code>(answer[i], answer[j])</code> of elements in this subset satisfies:</p>

<ul>
	<li><code>answer[i] % answer[j] == 0</code>, or</li>
	<li><code>answer[j] % answer[i] == 0</code></li>
</ul>

<p>If there are multiple solutions, return any of them.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> [1,3] is also accepted.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,4,8]
<strong>Output:</strong> [1,2,4,8]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 2 * 10<sup>9</sup></code></li>
	<li>All the integers in <code>nums</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn largest_divisible_subset(mut black_nums: Vec<i32>) -> Vec<i32> { black_nums.sort(); let mut black_dp = vec![1; black_nums.len()]; let (mut black_max, mut black_idx) = (1, 0); for i in 1..black_nums.len() { for j in 0..i { if black_nums[i] % black_nums[j] == 0 { black_dp[i] = black_dp[i].max(black_dp[j] + 1); } } if black_dp[i] > black_max { black_max = black_dp[i]; black_idx = i; } } let mut black_res = vec![]; let mut black_curr = black_nums[black_idx]; for i in (0..=black_idx).rev() { if black_curr % black_nums[i] == 0 && black_dp[i] == black_max { black_res.push(black_nums[i]); black_curr = black_nums[i]; black_max -= 1; } } black_res } }
```