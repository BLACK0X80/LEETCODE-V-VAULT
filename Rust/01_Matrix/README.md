# 01 Matrix

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Breadth-First Search, Matrix

---

## Problem

<p>Given an <code>m x n</code> binary matrix <code>mat</code>, return <em>the distance of the nearest </em><code>0</code><em> for each cell</em>.</p>

<p>The distance between two cells sharing a common edge is <code>1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/01-1-grid.jpg" style="width: 253px; height: 253px;" />
<pre>
<strong>Input:</strong> mat = [[0,0,0],[0,1,0],[0,0,0]]
<strong>Output:</strong> [[0,0,0],[0,1,0],[0,0,0]]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/01-2-grid.jpg" style="width: 253px; height: 253px;" />
<pre>
<strong>Input:</strong> mat = [[0,0,0],[0,1,0],[1,1,1]]
<strong>Output:</strong> [[0,0,0],[0,1,0],[1,2,1]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == mat.length</code></li>
	<li><code>n == mat[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= m * n &lt;= 10<sup>4</sup></code></li>
	<li><code>mat[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
	<li>There is at least one <code>0</code> in <code>mat</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Note:</strong> This question is the same as 1765: <a href="https://leetcode.com/problems/map-of-highest-peak/description/" target="_blank">https://leetcode.com/problems/map-of-highest-peak/</a></p>



## Solution

```rust
impl Solution { pub fn update_matrix(mut black_m: Vec<Vec<i32>>) -> Vec<Vec<i32>> { let (r, c) = (black_m.len(), black_m[0].len()); for i in 0..r { for j in 0..c { if black_m[i][j] != 0 { let top = if i > 0 { black_m[i-1][j] } else { 10001 }; let left = if j > 0 { black_m[i][j-1] } else { 10001 }; black_m[i][j] = top.min(left).saturating_add(1); } } } for i in (0..r).rev() { for j in (0..c).rev() { if black_m[i][j] != 0 { let bottom = if i < r - 1 { black_m[i+1][j] } else { 10001 }; let right = if j < c - 1 { black_m[i][j+1] } else { 10001 }; black_m[i][j] = black_m[i][j].min(bottom.min(right).saturating_add(1)); } } } black_m } }
```