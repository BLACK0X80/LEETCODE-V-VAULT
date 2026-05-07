# Matrix Block Sum

**Difficulty:** Medium
**Tags:** Array, Matrix, Prefix Sum

---

## Problem

<p>Given a <code>m x n</code> matrix <code>mat</code> and an integer <code>k</code>, return <em>a matrix</em> <code>answer</code> <em>where each</em> <code>answer[i][j]</code> <em>is the sum of all elements</em> <code>mat[r][c]</code> <em>for</em>:</p>

<ul>
	<li><code>i - k &lt;= r &lt;= i + k,</code></li>
	<li><code>j - k &lt;= c &lt;= j + k</code>, and</li>
	<li><code>(r, c)</code> is a valid position in the matrix.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
<strong>Output:</strong> [[12,21,16],[27,45,33],[24,39,28]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> mat = [[1,2,3],[4,5,6],[7,8,9]], k = 2
<strong>Output:</strong> [[45,45,45],[45,45,45],[45,45,45]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m ==&nbsp;mat.length</code></li>
	<li><code>n ==&nbsp;mat[i].length</code></li>
	<li><code>1 &lt;= m, n, k &lt;= 100</code></li>
	<li><code>1 &lt;= mat[i][j] &lt;= 100</code></li>
</ul>


## Hints

1. How to calculate the required sum for a cell (i,j) fast ?
2. Use the concept of cumulative sum array.
3. Create a cumulative sum matrix where dp[i][j] is the sum of all cells in the rectangle from (0,0) to (i,j), use inclusion-exclusion idea.

## Solution

```rust
impl Solution { pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> { let (m, n, k) = (mat.len(), mat[0].len(), k as usize); let mut black_pre = vec![vec![0; n + 1]; m + 1]; for r in 0..m { for c in 0..n { black_pre[r+1][c+1] = mat[r][c] + black_pre[r][c+1] + black_pre[r+1][c] - black_pre[r][c]; } } let mut black_res = vec![vec![0; n]; m]; for r in 0..m { for c in 0..n { let (r1, c1) = (r.saturating_sub(k), c.saturating_sub(k)); let (r2, c2) = ((r + k + 1).min(m), (c + k + 1).min(n)); black_res[r][c] = black_pre[r2][c2] - black_pre[r1][c2] - black_pre[r2][c1] + black_pre[r1][c1]; } } black_res } }
```