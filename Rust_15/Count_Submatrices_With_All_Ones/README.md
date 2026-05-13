# Count Submatrices With All Ones

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Stack, Matrix, Monotonic Stack

---

## Problem

<p>Given an <code>m x n</code> binary matrix <code>mat</code>, <em>return the number of <strong>submatrices</strong> that have all ones</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/10/27/ones1-grid.jpg" style="width: 244px; height: 245px;" />
<pre>
<strong>Input:</strong> mat = [[1,0,1],[1,1,0],[1,1,0]]
<strong>Output:</strong> 13
<strong>Explanation:</strong> 
There are 6 rectangles of side 1x1.
There are 2 rectangles of side 1x2.
There are 3 rectangles of side 2x1.
There is 1 rectangle of side 2x2. 
There is 1 rectangle of side 3x1.
Total number of rectangles = 6 + 2 + 3 + 1 + 1 = 13.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/10/27/ones2-grid.jpg" style="width: 324px; height: 245px;" />
<pre>
<strong>Input:</strong> mat = [[0,1,1,0],[0,1,1,1],[1,1,1,0]]
<strong>Output:</strong> 24
<strong>Explanation:</strong> 
There are 8 rectangles of side 1x1.
There are 5 rectangles of side 1x2.
There are 2 rectangles of side 1x3. 
There are 4 rectangles of side 2x1.
There are 2 rectangles of side 2x2. 
There are 2 rectangles of side 3x1. 
There is 1 rectangle of side 3x2. 
Total number of rectangles = 8 + 5 + 2 + 4 + 2 + 2 + 1 = 24.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m, n &lt;= 150</code></li>
	<li><code>mat[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>


## Hints

1. For each row i, create an array nums where:  if mat[i][j] == 0 then nums[j] = 0 else nums[j] = nums[j-1] +1.
2. In the row i, number of rectangles between column j and k(inclusive) and ends in row i, is equal to SUM(min(nums[j, .. idx])) where idx go from j to k.  Expected solution is O(n^3).

## Solution

```rust
impl Solution { pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 { let (black_r, black_c, mut black_res) = (mat.len(), mat[0].len(), 0); let mut black_h = vec![0; black_c]; for black_i in 0..black_r { for black_j in 0..black_c { black_h[black_j] = if mat[black_i][black_j] == 1 { black_h[black_j] + 1 } else { 0 }; let mut black_min_h = black_h[black_j]; for black_k in (0..=black_j).rev() { black_min_h = black_min_h.min(black_h[black_k]); if black_min_h == 0 { break; } black_res += black_min_h; } } } black_res } }
```