# Shortest Unsorted Continuous Subarray

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Stack, Greedy, Sorting, Monotonic Stack

---

## Problem

<p>Given an integer array <code>nums</code>, you need to find one <b>continuous subarray</b> such that if you only sort this subarray in non-decreasing order, then the whole array will be sorted in non-decreasing order.</p>

<p>Return <em>the shortest such subarray and output its length</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,6,4,8,10,9,15]
<strong>Output:</strong> 5
<strong>Explanation:</strong> You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 0
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1]
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>

<p>&nbsp;</p>
<strong>Follow up:</strong> Can you solve it in <code>O(n)</code> time complexity?


## Solution

```rust
impl Solution { pub fn find_unsorted_subarray(black_n: Vec<i32>) -> i32 { let (mut black_max, mut black_min, mut black_r, mut black_l) = (i32::MIN, i32::MAX, -1, -1); for i in 0..black_n.len() { black_max = black_max.max(black_n[i]); if black_n[i] < black_max { black_r = i as i32; } let j = black_n.len() - 1 - i; black_min = black_min.min(black_n[j]); if black_n[j] > black_min { black_l = j as i32; } } if black_r == -1 { 0 } else { black_r - black_l + 1 } } }
```