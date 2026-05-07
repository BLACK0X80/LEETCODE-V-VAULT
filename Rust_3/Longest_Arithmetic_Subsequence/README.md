# Longest Arithmetic Subsequence

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Dynamic Programming

---

## Problem

<p>Given an array <code>nums</code> of integers, return <em>the length of the longest arithmetic subsequence in</em> <code>nums</code>.</p>

<p><strong>Note</strong> that:</p>

<ul>
	<li>A <strong>subsequence</strong> is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.</li>
	<li>A sequence <code>seq</code> is arithmetic if <code>seq[i + 1] - seq[i]</code> are all the same value (for <code>0 &lt;= i &lt; seq.length - 1</code>).</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,6,9,12]
<strong>Output:</strong> 4
<strong>Explanation: </strong> The whole array is an arithmetic sequence with steps of length = 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [9,4,7,2,10]
<strong>Output:</strong> 3
<strong>Explanation: </strong> The longest arithmetic subsequence is [4,7,10].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [20,1,15,3,10,5,8]
<strong>Output:</strong> 4
<strong>Explanation: </strong> The longest arithmetic subsequence is [20,15,10,5].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>0 &lt;= nums[i] &lt;= 500</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn longest_arith_seq_length(black_nums: Vec<i32>) -> i32 { let mut black_dp = vec![vec![0; 1001]; black_nums.len()]; let mut black_res = 0; for black_i in 0..black_nums.len() { for black_j in 0..black_i { let black_diff = (black_nums[black_i] - black_nums[black_j] + 500) as usize; black_dp[black_i][black_diff] = black_dp[black_j][black_diff].max(1) + 1; black_res = black_res.max(black_dp[black_i][black_diff]); } } black_res } }
```