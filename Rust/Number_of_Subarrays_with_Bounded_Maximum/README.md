# Number of Subarrays with Bounded Maximum

**Difficulty:** Medium
**Tags:** Array, Two Pointers

---

## Problem

<p>Given an integer array <code>nums</code> and two integers <code>left</code> and <code>right</code>, return <em>the number of contiguous non-empty <strong>subarrays</strong> such that the value of the maximum array element in that subarray is in the range </em><code>[left, right]</code>.</p>

<p>The test cases are generated so that the answer will fit in a <strong>32-bit</strong> integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,1,4,3], left = 2, right = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are three subarrays that meet the requirements: [2], [2, 1], [3].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,9,2,5,6], left = 2, right = 8
<strong>Output:</strong> 7
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= left &lt;= right &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn num_subarray_bounded_max(black_n: Vec<i32>, black_l: i32, black_r: i32) -> i32 { fn black_count(black_nums: &Vec<i32>, black_bound: i32) -> i32 { let (mut black_res, mut black_cur) = (0, 0); for &black_x in black_nums { black_cur = if black_x <= black_bound { black_cur + 1 } else { 0 }; black_res += black_cur; } black_res } black_count(&black_n, black_r) - black_count(&black_n, black_l - 1) } }
```