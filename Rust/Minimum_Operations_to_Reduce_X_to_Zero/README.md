# Minimum Operations to Reduce X to Zero

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Sliding Window, Prefix Sum

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>x</code>. In one operation, you can either remove the leftmost or the rightmost element from the array <code>nums</code> and subtract its value from <code>x</code>. Note that this <strong>modifies</strong> the array for future operations.</p>

<p>Return <em>the <strong>minimum number</strong> of operations to reduce </em><code>x</code> <em>to <strong>exactly</strong></em> <code>0</code> <em>if it is possible</em><em>, otherwise, return </em><code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,1,4,2,3], x = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> The optimal solution is to remove the last two elements to reduce x to zero.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,6,7,8,9], x = 4
<strong>Output:</strong> -1
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,2,20,1,1,3], x = 10
<strong>Output:</strong> 5
<strong>Explanation:</strong> The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= x &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Think in reverse; instead of finding the minimum prefix + suffix, find the maximum subarray.
2. Finding the maximum subarray is standard and can be done greedily.

## Solution

```rust
impl Solution { pub fn min_operations(black_n: Vec<i32>, black_x: i32) -> i32 { let (black_target, mut black_s, mut black_l, mut black_m) = (black_n.iter().sum::<i32>() - black_x, 0, 0, -1); if black_target < 0 { return -1; } if black_target == 0 { return black_n.len() as i32; } for black_r in 0..black_n.len() { black_s += black_n[black_r]; while black_s > black_target { black_s -= black_n[black_l]; black_l += 1; } if black_s == black_target { black_m = black_m.max((black_r - black_l + 1) as i32); } } if black_m == -1 { -1 } else { (black_n.len() as i32 - black_m) } } }
```