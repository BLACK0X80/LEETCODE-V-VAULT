# Increment Submatrices by One

**Difficulty:** Medium
**Tags:** Array, Matrix, Prefix Sum

---

## Problem

<p>You are given a positive integer <code>n</code>, indicating that we initially have an <code>n x n</code>&nbsp;<strong>0-indexed</strong> integer matrix <code>mat</code> filled with zeroes.</p>

<p>You are also given a 2D integer array <code>query</code>. For each <code>query[i] = [row1<sub>i</sub>, col1<sub>i</sub>, row2<sub>i</sub>, col2<sub>i</sub>]</code>, you should do the following operation:</p>

<ul>
	<li>Add <code>1</code> to <strong>every element</strong> in the submatrix with the <strong>top left</strong> corner <code>(row1<sub>i</sub>, col1<sub>i</sub>)</code> and the <strong>bottom right</strong> corner <code>(row2<sub>i</sub>, col2<sub>i</sub>)</code>. That is, add <code>1</code> to <code>mat[x][y]</code> for all <code>row1<sub>i</sub> &lt;= x &lt;= row2<sub>i</sub></code> and <code>col1<sub>i</sub> &lt;= y &lt;= col2<sub>i</sub></code>.</li>
</ul>

<p>Return<em> the matrix</em> <code>mat</code><em> after performing every query.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/11/24/p2example11.png" style="width: 531px; height: 121px;" />
<pre>
<strong>Input:</strong> n = 3, queries = [[1,1,2,2],[0,0,1,1]]
<strong>Output:</strong> [[1,1,0],[1,2,1],[0,1,1]]
<strong>Explanation:</strong> The diagram above shows the initial matrix, the matrix after the first query, and the matrix after the second query.
- In the first query, we add 1 to every element in the submatrix with the top left corner (1, 1) and bottom right corner (2, 2).
- In the second query, we add 1 to every element in the submatrix with the top left corner (0, 0) and bottom right corner (1, 1).
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/11/24/p2example22.png" style="width: 261px; height: 82px;" />
<pre>
<strong>Input:</strong> n = 2, queries = [[0,0,1,1]]
<strong>Output:</strong> [[1,1],[1,1]]
<strong>Explanation:</strong> The diagram above shows the initial matrix and the matrix after the first query.
- In the first query we add 1 to every element in the matrix.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 500</code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= row1<sub>i</sub> &lt;= row2<sub>i</sub> &lt; n</code></li>
	<li><code>0 &lt;= col1<sub>i</sub> &lt;= col2<sub>i</sub> &lt; n</code></li>
</ul>


## Hints

1. Imagine each row as a separate array. Instead of updating the whole submatrix together, we can use prefix sum to update each row separately.
2. For each query, iterate over the rows i in the range [row1, row2] and add 1 to prefix sum S[i][col1], and subtract 1 from S[i][col2 + 1].
3. After doing this operation for all the queries, update each row separately with S[i][j] = S[i][j] + S[i][j - 1].

## Solution

```rust
impl Solution { pub fn range_add_queries(black_n: i32, black_q: Vec<Vec<i32>>) -> Vec<Vec<i32>> { let black_sz = black_n as usize; let mut black_d = vec![vec![0; black_sz + 1]; black_sz + 1]; for q in black_q { let (r1, c1, r2, c2) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize); black_d[r1][c1] += 1; black_d[r1][c2+1] -= 1; black_d[r2+1][c1] -= 1; black_d[r2+1][c2+1] += 1; } for i in 0..black_sz { for j in 1..black_sz { black_d[i][j] += black_d[i][j-1]; } } for j in 0..black_sz { for i in 1..black_sz { black_d[i][j] += black_d[i-1][j]; } } black_d.into_iter().take(black_sz).map(|r| r.into_iter().take(black_sz).collect()).collect() } }
```