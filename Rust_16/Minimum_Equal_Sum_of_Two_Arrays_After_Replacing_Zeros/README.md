# Minimum Equal Sum of Two Arrays After Replacing Zeros

**Difficulty:** Medium
**Tags:** Array, Greedy

---

## Problem

<p>You are given two arrays <code>nums1</code> and <code>nums2</code> consisting of positive integers.</p>

<p>You have to replace <strong>all</strong> the <code>0</code>&#39;s in both arrays with <strong>strictly</strong> positive integers such that the sum of elements of both arrays becomes <strong>equal</strong>.</p>

<p>Return <em>the <strong>minimum</strong> equal sum you can obtain, or </em><code>-1</code><em> if it is impossible</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [3,2,0,1,0], nums2 = [6,5,0]
<strong>Output:</strong> 12
<strong>Explanation:</strong> We can replace 0&#39;s in the following way:
- Replace the two 0&#39;s in nums1 with the values 2 and 4. The resulting array is nums1 = [3,2,2,1,4].
- Replace the 0 in nums2 with the value 1. The resulting array is nums2 = [6,5,1].
Both arrays have an equal sum of 12. It can be shown that it is the minimum sum we can obtain.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums1 = [2,0,2,0], nums2 = [1,4]
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is impossible to make the sum of both arrays equal.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums1.length, nums2.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums1[i], nums2[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Consider we replace all the 0’s with 1’s on both arrays, the answer will be <code>-1</code> if there was no <code>0</code> in the array with the smaller sum of elements.
2. Otherwise, how can you update the value of exactly one of these <code>1</code>’s to make the sum of the two arrays equal?

## Solution

```rust
impl Solution { pub fn min_sum(black_n1: Vec<i32>, black_n2: Vec<i32>) -> i64 { let (mut black_s1, mut black_z1) = (0i64, 0i64); for &black_x in &black_n1 { if black_x == 0 { black_z1 += 1; } black_s1 += black_x as i64; } let (mut black_s2, mut black_z2) = (0i64, 0i64); for &black_x in &black_n2 { if black_x == 0 { black_z2 += 1; } black_s2 += black_x as i64; } let black_min1 = black_s1 + black_z1; let black_min2 = black_s2 + black_z2; if (black_min1 < black_min2 && black_z1 == 0) || (black_min2 < black_min1 && black_z2 == 0) { return -1; } black_min1.max(black_min2) } }
```