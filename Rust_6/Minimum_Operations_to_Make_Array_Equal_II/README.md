# Minimum Operations to Make Array Equal II

**Difficulty:** Medium
**Tags:** Array, Math, Greedy

---

## Problem

<p>You are given two integer arrays <code>nums1</code> and <code>nums2</code> of equal length <code>n</code> and an integer <code>k</code>. You can perform the following operation on <code>nums1</code>:</p>

<ul>
	<li>Choose two indexes <code>i</code> and <code>j</code> and increment <code>nums1[i]</code> by <code>k</code> and decrement <code>nums1[j]</code> by <code>k</code>. In other words, <code>nums1[i] = nums1[i] + k</code> and <code>nums1[j] = nums1[j] - k</code>.</li>
</ul>

<p><code>nums1</code> is said to be <strong>equal</strong> to <code>nums2</code> if for all indices <code>i</code> such that <code>0 &lt;= i &lt; n</code>, <code>nums1[i] == nums2[i]</code>.</p>

<p>Return <em>the <strong>minimum</strong> number of operations required to make </em><code>nums1</code><em> equal to </em><code>nums2</code>. If it is impossible to make them equal, return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [4,3,1,4], nums2 = [1,3,7,1], k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> In 2 operations, we can transform nums1 to nums2.
1<sup>st</sup> operation: i = 2, j = 0. After applying the operation, nums1 = [1,3,4,4].
2<sup>nd</sup> operation: i = 2, j = 3. After applying the operation, nums1 = [1,3,7,1].
One can prove that it is impossible to make arrays equal in fewer operations.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [3,8,5,2], nums2 = [2,4,1,6], k = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong> It can be proved that it is impossible to make the two arrays equal.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums1.length == nums2.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums1[i], nums2[j] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. What are the cases for which we cannot make nums1 == nums2?
2. For minimum moves, if nums1[i] < nums2[i], then we should never decrement nums1[i]. 
If nums1[i] > nums2[i], then we should never increment nums1[i].

## Solution

```rust
impl Solution { pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 { if k == 0 { return if nums1 == nums2 { 0 } else { -1 }; } let (mut black_pos, mut black_neg) = (0i64, 0i64); for i in 0..nums1.len() { let black_d = (nums1[i] - nums2[i]) as i64; if black_d % k as i64 != 0 { return -1; } if black_d > 0 { black_pos += black_d / k as i64; } else { black_neg += black_d.abs() / k as i64; } } if black_pos == black_neg { black_pos } else { -1 } } }
```