# Minimum Number of Operations to Make All Array Elements Equal to 1

**Difficulty:** Medium
**Tags:** Array, Math, Number Theory

---

## Problem

<p>You are given a <strong>0-indexed</strong>&nbsp;array <code>nums</code> consisting of <strong>positive</strong> integers. You can do the following operation on the array <strong>any</strong> number of times:</p>

<ul>
	<li>Select an index <code>i</code> such that <code>0 &lt;= i &lt; n - 1</code> and replace either of&nbsp;<code>nums[i]</code> or <code>nums[i+1]</code> with their gcd value.</li>
</ul>

<p>Return <em>the <strong>minimum</strong> number of operations to make all elements of </em><code>nums</code><em> equal to </em><code>1</code>. If it is impossible, return <code>-1</code>.</p>

<p>The gcd of two integers is the greatest common divisor of the two integers.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,6,3,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong> We can do the following operations:
- Choose index i = 2 and replace nums[2] with gcd(3,4) = 1. Now we have nums = [2,6,1,4].
- Choose index i = 1 and replace nums[1] with gcd(6,1) = 1. Now we have nums = [2,1,1,4].
- Choose index i = 0 and replace nums[0] with gcd(2,1) = 1. Now we have nums = [1,1,1,4].
- Choose index i = 2 and replace nums[3] with gcd(1,4) = 1. Now we have nums = [1,1,1,1].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,10,6,14]
<strong>Output:</strong> -1
<strong>Explanation:</strong> It can be shown that it is impossible to make all the elements equal to 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 50</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Note that if you have at least one occurrence of 1 in the array, then you can make all the other elements equal to 1 with one operation each.
2. Try finding the shortest subarray with a gcd equal to 1.

## Solution

```rust
impl Solution { pub fn min_operations(nums: Vec<i32>) -> i32 { let (black_n, mut black_c) = (nums.len(), nums.iter().filter(|&&x| x == 1).count()); if black_c > 0 { return (black_n - black_c) as i32; } let mut black_res = i32::MAX; for i in 0..black_n { let mut black_g = nums[i]; for j in i + 1..black_n { fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd(b, a % b) } } black_g = gcd(black_g, nums[j]); if black_g == 1 { black_res = black_res.min((j - i + black_n - 1) as i32); break; } } } if black_res == i32::MAX { -1 } else { black_res } } }
```