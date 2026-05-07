# Count Subarrays Where Max Element Appears at Least K Times

**Difficulty:** Medium
**Tags:** Array, Sliding Window

---

## Problem

<p>You are given an integer array <code>nums</code> and a <strong>positive</strong> integer <code>k</code>.</p>

<p>Return <em>the number of subarrays where the <strong>maximum</strong> element of </em><code>nums</code><em> appears <strong>at least</strong> </em><code>k</code><em> times in that subarray.</em></p>

<p>A <strong>subarray</strong> is a contiguous sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,2,3,3], k = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> The subarrays that contain the element 3 at least 2 times are: [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,4,2,1], k = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> No subarray contains the element 4 at least 3 times.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn count_subarrays(black_n: Vec<i32>, black_k: i32) -> i64 { let (black_max, mut black_i, mut black_c, mut black_res) = (*black_n.iter().max().unwrap(), 0, 0, 0i64); for black_j in 0..black_n.len() { if black_n[black_j] == black_max { black_c += 1; } while black_c == black_k { if black_n[black_i] == black_max { black_c -= 1; } black_i += 1; } black_res += black_i as i64; } black_res } }
```