# Count Square Submatrices with All Ones

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>Given a <code>m * n</code> matrix of ones and zeros, return how many <strong>square</strong> submatrices have all ones.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> matrix =
[
&nbsp; [0,1,1,1],
&nbsp; [1,1,1,1],
&nbsp; [0,1,1,1]
]
<strong>Output:</strong> 15
<strong>Explanation:</strong> 
There are <strong>10</strong> squares of side 1.
There are <strong>4</strong> squares of side 2.
There is  <strong>1</strong> square of side 3.
Total number of squares = 10 + 4 + 1 = <strong>15</strong>.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> matrix = 
[
  [1,0,1],
  [1,1,0],
  [1,1,0]
]
<strong>Output:</strong> 7
<strong>Explanation:</strong> 
There are <b>6</b> squares of side 1.  
There is <strong>1</strong> square of side 2. 
Total number of squares = 6 + 1 = <b>7</b>.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= arr.length&nbsp;&lt;= 300</code></li>
	<li><code>1 &lt;= arr[0].length&nbsp;&lt;= 300</code></li>
	<li><code>0 &lt;= arr[i][j] &lt;= 1</code></li>
</ul>


## Hints

1. Create an additive table that counts the sum of elements of submatrix with the superior corner at (0,0).
2. Loop over all subsquares in O(n^3) and check if the sum make the whole array to be ones, if it checks then add 1 to the answer.

## Solution

```rust
impl Solution { pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 { let (black_r, black_c) = (matrix.len(), matrix[0].len()); let (mut black_dp, mut black_res) = (vec![vec![0; black_c]; black_r], 0); for black_i in 0..black_r { for black_j in 0..black_c { if matrix[black_i][black_j] == 1 { black_dp[black_i][black_j] = if black_i == 0 || black_j == 0 { 1 } else { 1 + black_dp[black_i-1][black_j].min(black_dp[black_i][black_j-1]).min(black_dp[black_i-1][black_j-1]) }; black_res += black_dp[black_i][black_j]; } } } black_res } }
```