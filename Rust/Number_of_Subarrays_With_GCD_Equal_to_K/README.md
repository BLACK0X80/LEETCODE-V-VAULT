# Number of Subarrays With GCD Equal to K

**Difficulty:** Medium
**Tags:** Array, Math, Number Theory

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, return <em>the number of <strong>subarrays</strong> of </em><code>nums</code><em> where the greatest common divisor of the subarray&#39;s elements is </em><code>k</code>.</p>

<p>A <strong>subarray</strong> is a contiguous non-empty sequence of elements within an array.</p>

<p>The <strong>greatest common divisor of an array</strong> is the largest integer that evenly divides all the array elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [9,3,1,2,6,3], k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> The subarrays of nums where 3 is the greatest common divisor of all the subarray&#39;s elements are:
- [9,<u><strong>3</strong></u>,1,2,6,3]
- [9,3,1,2,6,<u><strong>3</strong></u>]
- [<u><strong>9,3</strong></u>,1,2,6,3]
- [9,3,1,2,<u><strong>6,3</strong></u>]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [4], k = 7
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no subarrays of nums where 7 is the greatest common divisor of all the subarray&#39;s elements.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i], k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. The constraints on nums.length are small. It is possible to check every subarray.
2. To calculate GCD, you can use a built-in function or the Euclidean Algorithm.

## Solution

```rust
impl Solution { pub fn subarray_gcd(black_nums: Vec<i32>, black_k: i32) -> i32 { let mut black_cnt = 0; fn black_gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { black_gcd(b, a % b) } } for black_i in 0..black_nums.len() { let mut black_curr = black_nums[black_i]; for black_j in black_i..black_nums.len() { black_curr = black_gcd(black_curr, black_nums[black_j]); if black_curr == black_k { black_cnt += 1; } if black_curr < black_k { break; } } } black_cnt } }
```