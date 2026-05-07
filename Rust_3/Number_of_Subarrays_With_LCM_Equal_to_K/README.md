# Number of Subarrays With LCM Equal to K

**Difficulty:** Medium
**Tags:** Array, Math, Number Theory

---

## Problem

<p>Given an integer array <code>nums</code> and an integer <code>k</code>, return <em>the number of <strong>subarrays</strong> of </em><code>nums</code><em> where the least common multiple of the subarray&#39;s elements is </em><code>k</code>.</p>

<p>A <strong>subarray</strong> is a contiguous non-empty sequence of elements within an array.</p>

<p>The <strong>least common multiple of an array</strong> is the smallest positive integer that is divisible by all the array elements.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,6,2,7,1], k = 6
<strong>Output:</strong> 4
<strong>Explanation:</strong> The subarrays of nums where 6 is the least common multiple of all the subarray&#39;s elements are:
- [<u><strong>3</strong></u>,<u><strong>6</strong></u>,2,7,1]
- [<u><strong>3</strong></u>,<u><strong>6</strong></u>,<u><strong>2</strong></u>,7,1]
- [3,<u><strong>6</strong></u>,2,7,1]
- [3,<u><strong>6</strong></u>,<u><strong>2</strong></u>,7,1]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no subarrays of nums where 2 is the least common multiple of all the subarray&#39;s elements.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i], k &lt;= 1000</code></li>
</ul>


## Hints

1. The constraints on nums.length are small. It is possible to check every subarray.
2. To calculate LCM, you can use a built-in function or the formula lcm(a, b) = a * b / gcd(a, b).
3. As you calculate the LCM of more numbers, it can only become greater. Once it becomes greater than k, you know that any larger subarrays containing all the current elements will not work.

## Solution

```rust
impl Solution { pub fn subarray_lcm(black_nums: Vec<i32>, black_k: i32) -> i32 { let mut black_cnt = 0; fn black_gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { black_gcd(b, a % b) } } fn black_lcm(a: i64, b: i64) -> i64 { (a * b) / black_gcd(a, b) } for black_i in 0..black_nums.len() { let mut black_curr = black_nums[black_i] as i64; for black_j in black_i..black_nums.len() { black_curr = black_lcm(black_curr, black_nums[black_j] as i64); if black_curr == black_k as i64 { black_cnt += 1; } if black_curr > black_k as i64 { break; } } } black_cnt } }
```