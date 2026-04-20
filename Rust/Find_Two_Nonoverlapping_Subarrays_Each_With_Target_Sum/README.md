# Find Two Non-overlapping Sub-arrays Each With Target Sum

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Dynamic Programming, Sliding Window

---

## Problem

<p>You are given an array of integers <code>arr</code> and an integer <code>target</code>.</p>

<p>You have to find <strong>two non-overlapping sub-arrays</strong> of <code>arr</code> each with a sum equal <code>target</code>. There can be multiple answers so you have to find an answer where the sum of the lengths of the two sub-arrays is <strong>minimum</strong>.</p>

<p>Return <em>the minimum sum of the lengths</em> of the two required sub-arrays, or return <code>-1</code> if you cannot find such two sub-arrays.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [3,2,2,4,3], target = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> Only two sub-arrays have sum = 3 ([3] and [3]). The sum of their lengths is 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [7,3,4,7], target = 7
<strong>Output:</strong> 2
<strong>Explanation:</strong> Although we have three non-overlapping sub-arrays of sum = 7 ([7], [3,4] and [7]), but we will choose the first and third sub-arrays as the sum of their lengths is 2.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [4,3,2,6,2,3,4], target = 6
<strong>Output:</strong> -1
<strong>Explanation:</strong> We have only one sub-array of sum = 6.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= arr[i] &lt;= 1000</code></li>
	<li><code>1 &lt;= target &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. Let's create two arrays prefix and suffix where prefix[i] is the minimum length of sub-array ends before i and has sum = k, suffix[i] is the minimum length of sub-array starting at or after i and has sum = k.
2. The answer we are searching for is min(prefix[i] + suffix[i]) for all values of i from 0 to n-1 where n == arr.length.
3. If you are still stuck with how to build prefix and suffix, you can store for each index i the length of the sub-array starts at i and has sum = k or infinity otherwise, and you can use it to build both prefix and suffix.

## Solution

```rust
impl Solution { pub fn min_sum_of_lengths(black_a: Vec<i32>, black_t: i32) -> i32 { let (mut black_m, mut black_pre, mut black_s, mut black_l, mut black_res) = (vec![i32::MAX; black_a.len()], i32::MAX, 0, 0, i32::MAX); for black_r in 0..black_a.len() { black_s += black_a[black_r]; while black_s > black_t { black_s -= black_a[black_l]; black_l += 1; } if black_s == black_t { let black_len = (black_r - black_l + 1) as i32; if black_l > 0 && black_m[black_l - 1] != i32::MAX { black_res = black_res.min(black_len + black_m[black_l - 1]); } black_pre = black_pre.min(black_len); } black_m[black_r] = black_pre; } if black_res == i32::MAX { -1 } else { black_res } } }
```