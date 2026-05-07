# Partition Equal Subset Sum

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>Given an integer array <code>nums</code>, return <code>true</code> <em>if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,5,11,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> The array can be partitioned as [1, 5, 5] and [11].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,5]
<strong>Output:</strong> false
<strong>Explanation:</strong> The array cannot be partitioned into equal sum subsets.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 200</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 100</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn can_partition(black_n: Vec<i32>) -> bool { let black_s: i32 = black_n.iter().sum(); if black_s % 2 != 0 { return false; } let black_t = (black_s / 2) as usize; let mut black_dp = vec![false; black_t + 1]; black_dp[0] = true; for &x in &black_n { for i in (x as usize..=black_t).rev() { if black_dp[i - x as usize] { black_dp[i] = true; } } if black_dp[black_t] { return true; } } black_dp[black_t] } }
```