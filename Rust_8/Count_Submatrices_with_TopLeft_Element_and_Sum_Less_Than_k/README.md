# Count Submatrices with Top-Left Element and Sum Less Than k

**Difficulty:** Medium
**Tags:** Array, Matrix, Prefix Sum

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer matrix <code>grid</code> and an integer <code>k</code>.</p>

<p>Return <em>the <strong>number</strong> of <span data-keyword="submatrix">submatrices</span> that contain the top-left element of the</em> <code>grid</code>, <em>and have a sum less than or equal to </em><code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/01/01/example1.png" style="padding: 10px; background: #fff; border-radius: .5rem;" />
<pre>
<strong>Input:</strong> grid = [[7,6,3],[6,6,1]], k = 18
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are only 4 submatrices, shown in the image above, that contain the top-left element of grid, and have a sum less than or equal to 18.</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/01/01/example21.png" style="padding: 10px; background: #fff; border-radius: .5rem;" />
<pre>
<strong>Input:</strong> grid = [[7,2,9],[1,5,0],[2,6,6]], k = 20
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are only 6 submatrices, shown in the image above, that contain the top-left element of grid, and have a sum less than or equal to 20.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length </code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= n, m &lt;= 1000 </code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 1000</code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn count_submatrices(mut black_g: Vec<Vec<i32>>, black_k: i32) -> i32 { let (black_m, black_n, mut black_res) = (black_g.len(), black_g[0].len(), 0); for i in 0..black_m { for j in 0..black_n { if i > 0 { black_g[i][j] += black_g[i-1][j]; } if j > 0 { black_g[i][j] += black_g[i][j-1]; } if i > 0 && j > 0 { black_g[i][j] -= black_g[i-1][j-1]; } if black_g[i][j] <= black_k { black_res += 1; } else { break; } } } black_res } }
```