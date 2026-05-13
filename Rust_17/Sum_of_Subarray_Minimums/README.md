# Sum of Subarray Minimums

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Stack, Monotonic Stack

---

## Problem

<p>Given an array of integers arr, find the sum of <code>min(b)</code>, where <code>b</code> ranges over every (contiguous) subarray of <code>arr</code>. Since the answer may be large, return the answer <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [3,1,2,4]
<strong>Output:</strong> 17
<strong>Explanation:</strong> 
Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4]. 
Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
Sum is 17.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [11,81,94,43,3]
<strong>Output:</strong> 444
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= arr[i] &lt;= 3 * 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn sum_subarray_mins(black_arr: Vec<i32>) -> i32 { let (n, black_mod) = (black_arr.len(), 1_000_000_007); let (mut black_left, mut black_right, mut black_stack) = (vec![0; n], vec![0; n], vec![]); for i in 0..n { while !black_stack.is_empty() && black_arr[*black_stack.last().unwrap()] >= black_arr[i] { black_stack.pop(); } black_left[i] = if black_stack.is_empty() { i + 1 } else { i - black_stack.last().unwrap() }; black_stack.push(i); } black_stack.clear(); for i in (0..n).rev() { while !black_stack.is_empty() && black_arr[*black_stack.last().unwrap()] > black_arr[i] { black_stack.pop(); } black_right[i] = if black_stack.is_empty() { n - i } else { black_stack.last().unwrap() - i }; black_stack.push(i); } black_arr.iter().enumerate().fold(0, |acc, (i, &v)| (acc + v as i64 * black_left[i] as i64 * black_right[i] as i64) % black_mod as i64) as i32 } }
```