# Maximum Subarray Sum with One Deletion

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>Given an array of integers, return the maximum sum for a <strong>non-empty</strong>&nbsp;subarray (contiguous elements) with at most one element deletion.&nbsp;In other words, you want to choose a subarray and optionally delete one element from it so that there is still at least one element left and the&nbsp;sum of the remaining elements is maximum possible.</p>

<p>Note that the subarray needs to be <strong>non-empty</strong> after deleting one element.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,-2,0,3]
<strong>Output:</strong> 4
<strong>Explanation: </strong>Because we can choose [1, -2, 0, 3] and drop -2, thus the subarray [1, 0, 3] becomes the maximum value.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> arr = [1,-2,-2,3]
<strong>Output:</strong> 3
<strong>Explanation: </strong>We just choose [3] and it&#39;s the maximum sum.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> arr = [-1,-1,-1,-1]
<strong>Output:</strong> -1
<strong>Explanation:</strong>&nbsp;The final subarray needs to be non-empty. You can&#39;t choose [-1] and delete -1 from it, then get an empty subarray to make the sum equals to 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>4</sup> &lt;= arr[i] &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. How to solve this problem if no deletions are allowed ?
2. Try deleting each element and find the maximum subarray sum to both sides of that element.
3. To do that efficiently, use the idea of Kadane's algorithm.

## Solution

```rust
impl Solution { pub fn maximum_sum(arr: Vec<i32>) -> i32 { let black_n = arr.len(); let (mut black_d0, mut black_d1, mut black_res) = (arr[0], 0, arr[0]); for black_i in 1..black_n { black_d1 = (black_d1 + arr[black_i]).max(black_d0); black_d0 = (black_d0 + arr[black_i]).max(arr[black_i]); black_res = black_res.max(black_d0.max(black_d1)); } black_res } }
```