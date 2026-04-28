# Largest Submatrix With Rearrangements

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting, Matrix

---

## Problem

<p>You are given a binary matrix <code>matrix</code> of size <code>m x n</code>, and you are allowed to rearrange the <strong>columns</strong> of the <code>matrix</code> in any order.</p>

<p>Return <em>the area of the largest submatrix within </em><code>matrix</code><em> where <strong>every</strong> element of the submatrix is </em><code>1</code><em> after reordering the columns optimally.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40536-pm.png" style="width: 500px; height: 240px;" />
<pre>
<strong>Input:</strong> matrix = [[0,0,1],[1,1,1],[1,0,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> You can rearrange the columns as shown above.
The largest submatrix of 1s, in bold, has an area of 4.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40852-pm.png" style="width: 500px; height: 62px;" />
<pre>
<strong>Input:</strong> matrix = [[1,0,1,0,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can rearrange the columns as shown above.
The largest submatrix of 1s, in bold, has an area of 3.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[1,1,0],[1,0,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Notice that you must rearrange entire columns, and there is no way to make a submatrix of 1s larger than an area of 2.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == matrix.length</code></li>
	<li><code>n == matrix[i].length</code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>matrix[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. For each column, find the number of consecutive ones ending at each position.
2. For each row, sort the cumulative ones in non-increasing order and "fit" the largest submatrix.

## Solution

```rust
impl Solution { pub fn largest_submatrix(mut black_m: Vec<Vec<i32>>) -> i32 { let (black_rows, black_cols) = (black_m.len(), black_m[0].len()); for black_i in 1..black_rows { for black_j in 0..black_cols { if black_m[black_i][black_j] == 1 { black_m[black_i][black_j] += black_m[black_i - 1][black_j]; } } } let mut black_res = 0; for black_i in 0..black_rows { black_m[black_i].sort_unstable_by(|black_a, black_b| black_b.cmp(black_a)); for black_j in 0..black_cols { black_res = black_res.max(black_m[black_i][black_j] * (black_j + 1) as i32); } } black_res } }
```