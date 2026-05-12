# Partition Array into Disjoint Intervals

**Difficulty:** Medium
**Tags:** Array

---

## Problem

<p>Given an integer array <code>nums</code>, partition it into two (contiguous) subarrays <code>left</code> and <code>right</code> so that:</p>

<ul>
	<li>Every element in <code>left</code> is less than or equal to every element in <code>right</code>.</li>
	<li><code>left</code> and <code>right</code> are non-empty.</li>
	<li><code>left</code> has the smallest possible size.</li>
</ul>

<p>Return <em>the length of </em><code>left</code><em> after such a partitioning</em>.</p>

<p>Test cases are generated such that partitioning exists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,0,3,8,6]
<strong>Output:</strong> 3
<strong>Explanation:</strong> left = [5,0,3], right = [8,6]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,1,0,6,12]
<strong>Output:</strong> 4
<strong>Explanation:</strong> left = [1,1,1,0], right = [6,12]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li>There is at least one valid answer for the given input.</li>
</ul>



## Solution

```rust
impl Solution { pub fn partition_disjoint(black_n: Vec<i32>) -> i32 { let (mut black_mx, mut black_cur_mx, mut black_pos) = (black_n[0], black_n[0], 0); for black_i in 1..black_n.len() { if black_n[black_i] < black_mx { black_pos = black_i; black_mx = black_cur_mx; } else { black_cur_mx = black_cur_mx.max(black_n[black_i]); } } black_pos as i32 + 1 } }
```