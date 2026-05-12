# Partition Array for Maximum Sum

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>Given an integer array <code>arr</code>, partition the array into (contiguous) subarrays of length <strong>at most</strong> <code>k</code>. After partitioning, each subarray has their values changed to become the maximum value of that subarray.</p>

<p>Return <em>the largest sum of the given array after partitioning. Test cases are generated so that the answer fits in a <strong>32-bit</strong> integer.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,15,7,9,2,5,10], k = 3
<strong>Output:</strong> 84
<strong>Explanation:</strong> arr becomes [15,15,15,9,10,10,10]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,4,1,5,7,3,6,1,9,9,3], k = 4
<strong>Output:</strong> 83
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [1], k = 1
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 500</code></li>
	<li><code>0 &lt;= arr[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= k &lt;= arr.length</code></li>
</ul>


## Hints

1. Think dynamic programming:  dp[i] will be the answer for array A[0], ..., A[i-1].
2. For j = 1 .. k that keeps everything in bounds, dp[i] is the maximum of dp[i-j] + max(A[i-1], ..., A[i-j]) * j .

## Solution

```rust
impl Solution { pub fn max_sum_after_partitioning(black_arr: Vec<i32>, black_k: i32) -> i32 { let black_n = black_arr.len(); let mut black_dp = vec![0; black_n + 1]; for black_i in 1..=black_n { let mut black_max = 0; for black_j in 1..=black_k as usize { if black_i >= black_j { black_max = black_max.max(black_arr[black_i - black_j]); black_dp[black_i] = black_dp[black_i].max(black_dp[black_i - black_j] + black_max * black_j as i32); } } } black_dp[black_n] } }
```