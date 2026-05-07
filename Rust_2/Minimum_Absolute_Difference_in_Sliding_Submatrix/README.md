# Minimum Absolute Difference in Sliding Submatrix

**Difficulty:** Medium
**Tags:** Array, Sorting, Matrix

---

## Problem

<p>You are given an <code>m x n</code> integer matrix <code>grid</code> and an integer <code>k</code>.</p>

<p>For every contiguous <code>k x k</code> <strong>submatrix</strong> of <code>grid</code>, compute the <strong>minimum absolute</strong> difference between any two <strong>distinct</strong> values within that <strong>submatrix</strong>.</p>

<p>Return a 2D array <code>ans</code> of size <code>(m - k + 1) x (n - k + 1)</code>, where <code>ans[i][j]</code> is the minimum absolute difference in the submatrix whose top-left corner is <code>(i, j)</code> in <code>grid</code>.</p>

<p><strong>Note</strong>: If all elements in the submatrix have the same value, the answer will be 0.</p>
A submatrix <code>(x1, y1, x2, y2)</code> is a matrix that is formed by choosing all cells <code>matrix[x][y]</code> where <code>x1 &lt;= x &lt;= x2</code> and <code>y1 &lt;= y &lt;= y2</code>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,8],[3,-2]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[[2]]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There is only one possible <code>k x k</code> submatrix: <code><span class="example-io">[[1, 8], [3, -2]]</span></code><span class="example-io">.</span></li>
	<li>Distinct values in the submatrix are<span class="example-io"> <code>[1, 8, 3, -2]</code>.</span></li>
	<li>The minimum absolute difference in the submatrix is <code>|1 - 3| = 2</code>. Thus, the answer is <code>[[2]]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[3,-1]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">[[0,0]]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Both <code>k x k</code> submatrix has only one distinct element.</li>
	<li>Thus, the answer is <code>[[0, 0]]</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,-2,3],[2,3,5]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[[1,2]]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>There are two possible <code>k &times; k</code> submatrix:

	<ul>
		<li>Starting at <code>(0, 0)</code>: <code>[[1, -2], [2, 3]]</code>.

		<ul>
			<li>Distinct values in the submatrix are <code>[1, -2, 2, 3]</code>.</li>
			<li>The minimum absolute difference in the submatrix is <code>|1 - 2| = 1</code>.</li>
		</ul>
		</li>
		<li>Starting at <code>(0, 1)</code>: <code>[[-2, 3], [3, 5]]</code>.
		<ul>
			<li>Distinct values in the submatrix are <code>[-2, 3, 5]</code>.</li>
			<li>The minimum absolute difference in the submatrix is <code>|3 - 5| = 2</code>.</li>
		</ul>
		</li>
	</ul>
	</li>
	<li>Thus, the answer is <code>[[1, 2]]</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m == grid.length &lt;= 30</code></li>
	<li><code>1 &lt;= n == grid[i].length &lt;= 30</code></li>
	<li><code>-10<sup>5</sup> &lt;= grid[i][j] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= min(m, n)</code></li>
</ul>


## Hints

1. Use bruteforce over the submatrices

## Solution

```rust
impl Solution { pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> { let (black_m, black_n, black_k) = (grid.len(), grid[0].len(), k as usize); (0..=black_m-black_k).map(|black_i| (0..=black_n-black_k).map(|black_j| { let mut black_v = vec![]; for black_r in black_i..black_i+black_k { for black_c in black_j..black_j+black_k { black_v.push(grid[black_r][black_c]); } } black_v.sort_unstable(); black_v.dedup(); let mut black_res = i32::MAX; for black_idx in 0..black_v.len().saturating_sub(1) { black_res = black_res.min((black_v[black_idx+1] - black_v[black_idx]).abs()); } if black_v.len() < 2 { 0 } else { black_res } }).collect()).collect() } }
```