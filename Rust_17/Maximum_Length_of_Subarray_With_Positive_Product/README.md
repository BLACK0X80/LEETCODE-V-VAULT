# Maximum Length of Subarray With Positive Product

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Greedy

---

## Problem

<p>Given an array of integers <code>nums</code>, find the maximum length of a subarray where the product of all its elements is positive.</p>

<p>A subarray of an array is a consecutive sequence of zero or more values taken out of that array.</p>

<p>Return <em>the maximum length of a subarray with positive product</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,-2,-3,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The array nums already has a positive product of 24.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [0,1,-2,-3,-4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest subarray with positive product is [1,-2,-3] which has a product of 6.
Notice that we cannot include 0 in the subarray since that&#39;ll make the product 0 which is not positive.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [-1,-2,-3,0,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The longest subarray with positive product is [-1,-2] or [-2,-3].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Split the whole array into subarrays by zeroes since a subarray with positive product cannot contain any zero.
2. If the subarray has even number of negative numbers, the whole subarray has positive product.
3. Otherwise, we have two choices, either - remove the prefix till the first negative element in this subarray, or remove the suffix starting from the last negative element in this subarray.

## Solution

```rust
impl Solution { pub fn get_max_len(black_n: Vec<i32>) -> i32 { let (mut black_pos, mut black_neg, mut black_res) = (0, 0, 0); for &black_x in &black_n { if black_x == 0 { black_pos = 0; black_neg = 0; } else if black_x > 0 { black_pos += 1; black_neg = if black_neg > 0 { black_neg + 1 } else { 0 }; } else { let black_tmp = black_pos; black_pos = if black_neg > 0 { black_neg + 1 } else { 0 }; black_neg = black_tmp + 1; } black_res = black_res.max(black_pos); } black_res } }
```