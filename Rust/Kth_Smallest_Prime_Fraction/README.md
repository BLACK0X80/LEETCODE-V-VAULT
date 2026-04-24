# K-th Smallest Prime Fraction

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Binary Search, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given a sorted integer array <code>arr</code> containing <code>1</code> and <strong>prime</strong> numbers, where all the integers of <code>arr</code> are unique. You are also given an integer <code>k</code>.</p>

<p>For every <code>i</code> and <code>j</code> where <code>0 &lt;= i &lt; j &lt; arr.length</code>, we consider the fraction <code>arr[i] / arr[j]</code>.</p>

<p>Return <em>the</em> <code>k<sup>th</sup></code> <em>smallest fraction considered</em>. Return your answer as an array of integers of size <code>2</code>, where <code>answer[0] == arr[i]</code> and <code>answer[1] == arr[j]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,3,5], k = 3
<strong>Output:</strong> [2,5]
<strong>Explanation:</strong> The fractions to be considered in sorted order are:
1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
The third fraction is 2/5.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,7], k = 1
<strong>Output:</strong> [1,7]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= arr.length &lt;= 1000</code></li>
	<li><code>1 &lt;= arr[i] &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>arr[0] == 1</code></li>
	<li><code>arr[i]</code> is a <strong>prime</strong> number for <code>i &gt; 0</code>.</li>
	<li>All the numbers of <code>arr</code> are <strong>unique</strong> and sorted in <strong>strictly increasing</strong> order.</li>
	<li><code>1 &lt;= k &lt;= arr.length * (arr.length - 1) / 2</code></li>
</ul>

<p>&nbsp;</p>
<strong>Follow up:</strong> Can you solve the problem with better than <code>O(n<sup>2</sup>)</code> complexity?


## Solution

```rust
impl Solution { pub fn kth_smallest_prime_fraction(black_arr: Vec<i32>, black_k: i32) -> Vec<i32> { let (mut black_l, mut black_r) = (0.0, 1.0); for _ in 0..100 { let (black_m, mut black_count, mut black_p, mut black_q) = ((black_l + black_r) / 2.0, 0, 0, 1); let mut black_j = 1; for black_i in 0..black_arr.len() { while black_j < black_arr.len() && black_arr[black_i] as f64 > black_m * black_arr[black_j] as f64 { black_j += 1; } black_count += black_arr.len() - black_j; if black_j < black_arr.len() && black_arr[black_i] as f64 * black_q as f64 > black_p as f64 * black_arr[black_j] as f64 { black_p = black_arr[black_i]; black_q = black_arr[black_j]; } } if black_count == black_k as usize { return vec![black_p, black_q]; } if black_count < black_k as usize { black_l = black_m; } else { black_r = black_m; } } vec![] } }
```