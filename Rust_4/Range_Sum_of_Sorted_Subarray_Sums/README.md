# Range Sum of Sorted Subarray Sums

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Sorting, Prefix Sum

---

## Problem

<p>You are given the array <code>nums</code> consisting of <code>n</code> positive integers. You computed the sum of all non-empty continuous subarrays from the array and then sorted them in non-decreasing order, creating a new array of <code>n * (n + 1) / 2</code> numbers.</p>

<p><em>Return the sum of the numbers from index </em><code>left</code><em> to index </em><code>right</code> (<strong>indexed from 1</strong>)<em>, inclusive, in the new array. </em>Since the answer can be a huge number return it modulo <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], n = 4, left = 1, right = 5
<strong>Output:</strong> 13 
<strong>Explanation:</strong> All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4. After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13. 
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], n = 4, left = 3, right = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The given array is the same as example 1. We have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 3 to ri = 4 is 3 + 3 = 6.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4], n = 4, left = 1, right = 10
<strong>Output:</strong> 50
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == nums.length</code></li>
	<li><code>1 &lt;= nums.length &lt;= 1000</code></li>
	<li><code>1 &lt;= nums[i] &lt;= 100</code></li>
	<li><code>1 &lt;= left &lt;= right &lt;= n * (n + 1) / 2</code></li>
</ul>


## Hints

1. Compute all sums and save it in array.
2. Then just go from LEFT to RIGHT index and calculate answer modulo 1e9 + 7.

## Solution

```rust
impl Solution { pub fn range_sum(black_n: Vec<i32>, black_sz: i32, black_l: i32, black_r: i32) -> i32 { let mut black_s = Vec::with_capacity((black_sz * (black_sz + 1) / 2) as usize); for black_i in 0..black_n.len() { let mut black_curr = 0; for black_j in black_i..black_n.len() { black_curr += black_n[black_j]; black_s.push(black_curr); } } black_s.sort_unstable(); (black_s[(black_l - 1) as usize..black_r as usize].iter().fold(0i64, |black_acc, &black_x| black_acc + black_x as i64) % 1_000_000_007) as i32 } }
```