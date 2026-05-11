# Maximum Value at a Given Index in a Bounded Array

**Difficulty:** Medium
**Tags:** Math, Binary Search, Greedy

---

## Problem

<p>You are given three positive integers:&nbsp;<code>n</code>, <code>index</code>, and <code>maxSum</code>. You want to construct an array <code>nums</code> (<strong>0-indexed</strong>)<strong> </strong>that satisfies the following conditions:</p>

<ul>
	<li><code>nums.length == n</code></li>
	<li><code>nums[i]</code> is a <strong>positive</strong> integer where <code>0 &lt;= i &lt; n</code>.</li>
	<li><code>abs(nums[i] - nums[i+1]) &lt;= 1</code> where <code>0 &lt;= i &lt; n-1</code>.</li>
	<li>The sum of all the elements of <code>nums</code> does not exceed <code>maxSum</code>.</li>
	<li><code>nums[index]</code> is <strong>maximized</strong>.</li>
</ul>

<p>Return <code>nums[index]</code><em> of the constructed array</em>.</p>

<p>Note that <code>abs(x)</code> equals <code>x</code> if <code>x &gt;= 0</code>, and <code>-x</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 4, index = 2,  maxSum = 6
<strong>Output:</strong> 2
<strong>Explanation:</strong> nums = [1,2,<u><strong>2</strong></u>,1] is one array that satisfies all the conditions.
There are no arrays that satisfy all the conditions and have nums[2] == 3, so 2 is the maximum nums[2].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 6, index = 1,  maxSum = 10
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= maxSum &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= index &lt; n</code></li>
</ul>


## Hints

1. What if the problem was instead determining if you could generate a valid array with nums[index] == target?
2. To generate the array, set nums[index] to target, nums[index-i] to target-i, and nums[index+i] to target-i. Then, this will give the minimum possible sum, so check if the sum is less than or equal to maxSum.
3. n is too large to actually generate the array, so you can use the formula 1 + 2 + ... + n = n * (n+1) / 2 to quickly find the sum of nums[0...index] and nums[index...n-1].
4. Binary search for the target. If it is possible, then move the lower bound up. Otherwise, move the upper bound down.

## Solution

```rust
impl Solution { pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 { let (mut black_l, mut black_r, black_sum_fn) = (1, max_sum, |black_n: i64, black_idx: i64, black_v: i64| { let black_f = |black_len: i64, black_val: i64| { if black_len <= black_val { (black_val + black_val - black_len + 1) * black_len / 2 } else { (black_val + 1) * black_val / 2 + (black_len - black_val) } }; black_f(black_idx + 1, black_v) + black_f(black_n - black_idx, black_v) - black_v }); while black_l < black_r { let black_m = (black_l + black_r + 1) / 2; if black_sum_fn(n as i64, index as i64, black_m as i64) <= max_sum as i64 { black_l = black_m; } else { black_r = black_m - 1; } } black_l } }
```