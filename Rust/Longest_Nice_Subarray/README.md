# Longest Nice Subarray

**Difficulty:** Medium
**Tags:** Array, Bit Manipulation, Sliding Window

---

## Problem

<p>You are given an array <code>nums</code> consisting of <strong>positive</strong> integers.</p>

<p>We call a subarray of <code>nums</code> <strong>nice</strong> if the bitwise <strong>AND</strong> of every pair of elements that are in <strong>different</strong> positions in the subarray is equal to <code>0</code>.</p>

<p>Return <em>the length of the <strong>longest</strong> nice subarray</em>.</p>

<p>A <strong>subarray</strong> is a <strong>contiguous</strong> part of an array.</p>

<p><strong>Note</strong> that subarrays of length <code>1</code> are always considered nice.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,8,48,10]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest nice subarray is [3,8,48]. This subarray satisfies the conditions:
- 3 AND 8 = 0.
- 3 AND 48 = 0.
- 8 AND 48 = 0.
It can be proven that no longer nice subarray can be obtained, so we return 3.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,1,5,11,13]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The length of the longest nice subarray is 1. Any subarray of length 1 can be chosen.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. What is the maximum possible length of a nice subarray?
2. If two numbers have bitwise AND equal to zero, they do not have any common set bit. A number <code>x <= 10<sup>9</sup></code> only has 30 bits, hence the length of the longest nice subarray cannot exceed 30.

## Solution

```rust
impl Solution { pub fn longest_nice_subarray(black_n: Vec<i32>) -> i32 { let (mut black_l, mut black_mask, mut black_res) = (0, 0, 0); for black_r in 0..black_n.len() { while (black_mask & black_n[black_r]) != 0 { black_mask ^= black_n[black_l]; black_l += 1; } black_mask |= black_n[black_r]; black_res = black_res.max(black_r - black_l + 1); } black_res as i32 } }
```