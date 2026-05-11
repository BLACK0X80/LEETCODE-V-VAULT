# Longest Non-Decreasing Subarray After Replacing at Most One Element

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an integer array <code>nums</code>.</p>

<p>You are allowed to replace <strong>at most</strong> one element in the array with any other integer value of your choice.</p>

<p>Return the length of the <strong>longest non-decreasing <span data-keyword="subarray">subarray</span></strong> that can be obtained after performing at most one replacement.</p>

<p>An array is said to be <strong>non-decreasing</strong> if each element is greater than or equal to its previous one (if it exists).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,2,3,1,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Replacing <code>nums[3] = 1</code> with 3 gives the array [1, 2, 3, 3, 2].</p>

<p>The longest non-decreasing subarray is [1, 2, 3, 3], which has a length of 4.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,2,2,2,2]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>All elements in <code>nums</code> are equal, so it is already non-decreasing and the entire <code>nums</code> forms a subarray of length 5.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code>​​​​​​​</li>
</ul>


## Hints

1. Use prefix and suffix arrays
2. Create a prefix array where <code>pref[i]</code> denotes the maximum-length non-decreasing subarray ending at <code>i</code>
3. Create a suffix array where <code>suff[i]</code> denotes the maximum-length non-decreasing subarray starting at <code>i</code>
4. Let the initial maximum be <code>max(max(pref), max(suff))</code>
5. For each index <code>i</code>, try to combine the prefix ending at <code>i - 1</code> and the suffix starting at <code>i + 1</code> if possible; also consider the maximum without combining them

## Solution

```rust
impl Solution { pub fn longest_subarray(black_a: Vec<i32>) -> i32 { let black_n = black_a.len(); if black_n <= 1 { return black_n as i32; } let (mut black_l, mut black_r) = (vec![1; black_n], vec![1; black_n]); for i in 1..black_n { if black_a[i-1] <= black_a[i] { black_l[i] = black_l[i-1] + 1; } } for i in (0..black_n-1).rev() { if black_a[i] <= black_a[i+1] { black_r[i] = black_r[i+1] + 1; } } let mut black_res = black_n.min(black_l.iter().max().unwrap_or(&1) + 1) as i32; for i in 1..black_n-1 { if black_a[i-1] <= black_a[i+1] { black_res = black_res.max((black_l[i-1] + 1 + black_r[i+1]) as i32); } } black_res } }
```