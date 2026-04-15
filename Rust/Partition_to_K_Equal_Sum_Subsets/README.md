# Partition to K Equal Sum Subsets

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Backtracking, Bit Manipulation, Memoization, Bitmask

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, return <code>true</code> if it is possible to divide this array into <code>k</code> non-empty subsets whose sums are all equal.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [4,3,2,3,5,2,1], k = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> It is possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 3
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= nums.length &lt;= 16</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li>The frequency of each element is in the range <code>[1, 4]</code>.</li>
</ul>


## Hints

1. We can figure out what target each subset must sum to.  Then, let's recursively search, where at each call to our function, we choose which of k subsets the next value will join.

## Solution

```rust
impl Solution { pub fn can_partition_k_subsets(black_n: Vec<i32>, k: i32) -> bool { let sum: i32 = black_n.iter().sum(); if sum % k != 0 { return false; } let target = sum / k; let mut black_dp = vec![-1; 1 << black_n.len()]; black_dp[0] = 0; for mask in 0..(1 << black_n.len()) { if black_dp[mask] == -1 { continue; } for i in 0..black_n.len() { if mask & (1 << i) == 0 && black_dp[mask] + black_n[i] <= target { black_dp[mask | (1 << i)] = (black_dp[mask] + black_n[i]) % target; } } } black_dp[(1 << black_n.len()) - 1] == 0 } }
```