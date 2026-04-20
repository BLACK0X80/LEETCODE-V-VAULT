# Subarray Product Less Than K

**Difficulty:** Medium
**Tags:** Array, Binary Search, Sliding Window, Prefix Sum

---

## Problem

<p>Given an array of integers <code>nums</code> and an integer <code>k</code>, return <em>the number of contiguous subarrays where the product of all the elements in the subarray is strictly less than </em><code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,5,2,6], k = 100
<strong>Output:</strong> 8
<strong>Explanation:</strong> The 8 subarrays that have product less than 100 are:
[10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3], k = 0
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 1000</code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. For each j, let opt(j) be the smallest i so that nums[i] * nums[i+1] * ... * nums[j] is less than k.  opt is an increasing function.

## Solution

```rust
impl Solution { pub fn num_subarray_product_less_than_k(black_nums: Vec<i32>, black_k: i32) -> i32 { if black_k <= 1 { return 0; } let (mut black_p, mut black_r, mut black_l) = (1, 0, 0); for black_i in 0..black_nums.len() { black_p *= black_nums[black_i]; while black_p >= black_k { black_p /= black_nums[black_l]; black_l += 1; } black_r += (black_i - black_l + 1) as i32; } black_r } }
```