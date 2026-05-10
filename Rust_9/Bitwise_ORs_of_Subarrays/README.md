# Bitwise ORs of Subarrays

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Bit Manipulation

---

## Problem

<p>Given an integer array <code>arr</code>, return <em>the number of distinct bitwise ORs of all the non-empty subarrays of</em> <code>arr</code>.</p>

<p>The bitwise OR of a subarray is the bitwise OR of each integer in the subarray. The bitwise OR of a subarray of one integer is that integer.</p>

<p>A <strong>subarray</strong> is a contiguous non-empty sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one possible result: 0.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,1,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The possible subarrays are [1], [1], [2], [1, 1], [1, 2], [1, 1, 2].
These yield the results 1, 1, 2, 1, 3, 3.
There are 3 unique values, so the answer is 3.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,2,4]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The possible results are 1, 2, 3, 4, 6, and 7.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= arr[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Consider the the subarrays that end at index <code>i</code>. The number of unique bitwise OR for these subarrays is limited.

## Solution

```rust
use std::collections::HashSet; impl Solution { pub fn subarray_bitwise_o_rs(black_arr: Vec<i32>) -> i32 { let (mut black_ans, mut black_cur) = (HashSet::new(), HashSet::new()); for &x in &black_arr { let mut black_next = HashSet::from([x]); for &y in &black_cur { black_next.insert(x | y); } black_cur = black_next; for &val in &black_cur { black_ans.insert(val); } } black_ans.len() as i32 } }
```