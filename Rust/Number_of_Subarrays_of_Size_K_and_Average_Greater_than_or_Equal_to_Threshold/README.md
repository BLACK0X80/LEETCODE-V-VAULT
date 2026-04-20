# Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold

**Difficulty:** Medium
**Tags:** Array, Sliding Window

---

## Problem

<p>Given an array of integers <code>arr</code> and two integers <code>k</code> and <code>threshold</code>, return <em>the number of sub-arrays of size </em><code>k</code><em> and average greater than or equal to </em><code>threshold</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong> Sub-arrays [2,5,5],[5,5,5] and [5,5,8] have averages 4, 5 and 6 respectively. All other sub-arrays of size 3 have averages less than 4 (the threshold).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
<strong>Output:</strong> 6
<strong>Explanation:</strong> The first 6 sub-arrays of size 3 have averages greater than 5. Note that averages are not integers.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= arr[i] &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= k &lt;= arr.length</code></li>
	<li><code>0 &lt;= threshold &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Start with a window of size K and test its average against the threshold.
2. Keep moving the window by one element maintaining its size k until you cover the whole array. Count the number of windows that have an average greater than or equal to the threshold.

## Solution

```rust
impl Solution { pub fn num_of_subarrays(black_a: Vec<i32>, black_k: i32, black_t: i32) -> i32 { let (mut black_s, mut black_res, black_target) = (black_a.iter().take(black_k as usize).sum::<i32>(), 0, black_k * black_t); if black_s >= black_target { black_res += 1; } for black_i in black_k as usize..black_a.len() { black_s += black_a[black_i] - black_a[black_i - black_k as usize]; if black_s >= black_target { black_res += 1; } } black_res } }
```