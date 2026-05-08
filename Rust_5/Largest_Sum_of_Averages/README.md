# Largest Sum of Averages

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>k</code>. You can partition the array into <strong>at most</strong> <code>k</code> non-empty adjacent subarrays. The <strong>score</strong> of a partition is the sum of the averages of each subarray.</p>

<p>Note that the partition must use every integer in <code>nums</code>, and that the score is not necessarily an integer.</p>

<p>Return <em>the maximum <strong>score</strong> you can achieve of all the possible partitions</em>. Answers within <code>10<sup>-6</sup></code> of the actual answer will be accepted.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [9,1,2,3,9], k = 3
<strong>Output:</strong> 20.00000
<strong>Explanation:</strong> 
The best choice is to partition nums into [9], [1, 2, 3], [9]. The answer is 9 + (1 + 2 + 3) / 3 + 9 = 20.
We could have also partitioned nums into [9, 1], [2], [3, 9], for example.
That partition would lead to a score of 5 + 2 + 6 = 13, which is worse.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6,7], k = 4
<strong>Output:</strong> 20.50000
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 100</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= k &lt;= nums.length</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 { let n = nums.len(); let mut black_pre = vec![0.0; n + 1]; for i in 0..n { black_pre[i+1] = black_pre[i] + nums[i] as f64; } let mut black_dp = vec![0.0; n]; for i in 0..n { black_dp[i] = (black_pre[n] - black_pre[i]) / (n - i) as f64; } for _ in 0..k-1 { for i in 0..n { for j in i+1..n { black_dp[i] = black_dp[i].max((black_pre[j] - black_pre[i]) / (j - i) as f64 + black_dp[j]); } } } black_dp[0] } }
```