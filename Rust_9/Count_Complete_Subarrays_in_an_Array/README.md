# Count Complete Subarrays in an Array

**Difficulty:** Medium
**Tags:** Array, Hash Table, Sliding Window

---

## Problem

<p>You are given an array <code>nums</code> consisting of <strong>positive</strong> integers.</p>

<p>We call a subarray of an array <strong>complete</strong> if the following condition is satisfied:</p>

<ul>
	<li>The number of <strong>distinct</strong> elements in the subarray is equal to the number of distinct elements in the whole array.</li>
</ul>

<p>Return <em>the number of <strong>complete</strong> subarrays</em>.</p>

<p>A <strong>subarray</strong> is a contiguous non-empty part of an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,1,2,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The complete subarrays are the following: [1,3,1,2], [1,3,1,2,2], [3,1,2] and [3,1,2,2].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,5,5,5]
<strong>Output:</strong> 10
<strong>Explanation:</strong> The array consists only of the integer 5, so any subarray is complete. The number of subarrays that we can choose is 10.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 2000</code></li>
</ul>


## Hints

1. Let’s say k is the number of distinct elements in the array. Our goal is to find the number of subarrays with k distinct elements.
2. Since the constraints are small, you can check every subarray.

## Solution

```rust
impl Solution { pub fn count_complete_subarrays(black_n: Vec<i32>) -> i32 { let (black_total, mut black_m, mut black_i, mut black_res) = (black_n.iter().collect::<std::collections::HashSet<_>>().len(), std::collections::HashMap::new(), 0, 0); for black_j in 0..black_n.len() { *black_m.entry(black_n[black_j]).or_insert(0) += 1; while black_m.len() == black_total { *black_m.get_mut(&black_n[black_i]).unwrap() -= 1; if black_m[&black_n[black_i]] == 0 { black_m.remove(&black_n[black_i]); } black_i += 1; } black_res += black_i; } black_res as i32 } }
```