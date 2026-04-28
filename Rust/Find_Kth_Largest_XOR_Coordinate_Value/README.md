# Find Kth Largest XOR Coordinate Value

**Difficulty:** Medium
**Tags:** Array, Divide and Conquer, Bit Manipulation, Sorting, Heap (Priority Queue), Matrix, Prefix Sum, Quickselect

---

## Problem

<p>You are given a 2D <code>matrix</code> of size <code>m x n</code>, consisting of non-negative integers. You are also given an integer <code>k</code>.</p>

<p>The <strong>value</strong> of coordinate <code>(a, b)</code> of the matrix is the XOR of all <code>matrix[i][j]</code> where <code>0 &lt;= i &lt;= a &lt; m</code> and <code>0 &lt;= j &lt;= b &lt; n</code> <strong>(0-indexed)</strong>.</p>

<p>Find the <code>k<sup>th</sup></code> largest value <strong>(1-indexed)</strong> of all the coordinates of <code>matrix</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[5,2],[1,6]], k = 1
<strong>Output:</strong> 7
<strong>Explanation:</strong> The value of coordinate (0,1) is 5 XOR 2 = 7, which is the largest value.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[5,2],[1,6]], k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> The value of coordinate (0,0) is 5 = 5, which is the 2nd largest value.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> matrix = [[5,2],[1,6]], k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> The value of coordinate (1,0) is 5 XOR 1 = 4, which is the 3rd largest value.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == matrix.length</code></li>
	<li><code>n == matrix[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 1000</code></li>
	<li><code>0 &lt;= matrix[i][j] &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= k &lt;= m * n</code></li>
</ul>


## Hints

1. Use a 2D prefix sum to precalculate the xor-sum of the upper left submatrix.

## Solution

```rust
impl Solution { pub fn kth_largest_value(black_m: Vec<Vec<i32>>, black_k: i32) -> i32 { let (black_rows, black_cols) = (black_m.len(), black_m[0].len()); let mut black_x = vec![vec![0; black_cols]; black_rows]; let mut black_v = Vec::with_capacity(black_rows * black_cols); for black_i in 0..black_rows { for black_j in 0..black_cols { black_x[black_i][black_j] = black_m[black_i][black_j] ^ (if black_i > 0 { black_x[black_i - 1][black_j] } else { 0 }) ^ (if black_j > 0 { black_x[black_i][black_j - 1] } else { 0 }) ^ (if black_i > 0 && black_j > 0 { black_x[black_i - 1][black_j - 1] } else { 0 }); black_v.push(black_x[black_i][black_j]); } } black_v.sort_unstable_by(|black_a, black_b| black_b.cmp(black_a)); black_v[black_k as usize - 1] } }
```