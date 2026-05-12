# Sort Matrix by Diagonals

**Difficulty:** Medium
**Tags:** Array, Sorting, Matrix

---

## Problem

<p>You are given an <code>n x n</code> square matrix of integers <code>grid</code>. Return the matrix such that:</p>

<ul>
	<li>The diagonals in the <strong>bottom-left triangle</strong> (including the middle diagonal) are sorted in <strong>non-increasing order</strong>.</li>
	<li>The diagonals in the <strong>top-right triangle</strong> are sorted in <strong>non-decreasing order</strong>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,7,3],[9,8,2],[4,5,6]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[8,2,3],[9,6,7],[4,5,1]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/29/4052example1drawio.png" style="width: 461px; height: 181px;" /></p>

<p>The diagonals with a black arrow (bottom-left triangle) should be sorted in non-increasing order:</p>

<ul>
	<li><code>[1, 8, 6]</code> becomes <code>[8, 6, 1]</code>.</li>
	<li><code>[9, 5]</code> and <code>[4]</code> remain unchanged.</li>
</ul>

<p>The diagonals with a blue arrow (top-right triangle) should be sorted in non-decreasing order:</p>

<ul>
	<li><code>[7, 2]</code> becomes <code>[2, 7]</code>.</li>
	<li><code>[3]</code> remains unchanged.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0,1],[1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[2,1],[1,0]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/29/4052example2adrawio.png" style="width: 383px; height: 141px;" /></p>

<p>The diagonals with a black arrow must be non-increasing, so <code>[0, 2]</code> is changed to <code>[2, 0]</code>. The other diagonals are already in the correct order.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[1]]</span></p>

<p><strong>Explanation:</strong></p>

<p>Diagonals with exactly one element are already in order, so no changes are needed.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>grid.length == grid[i].length == n</code></li>
	<li><code>1 &lt;= n &lt;= 10</code></li>
	<li><code>-10<sup>5</sup> &lt;= grid[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Use a data structure to store all values in each diagonal.
2. Sort and replace them in the matrix.

## Solution

```rust
impl Solution { pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> { let n = grid.len(); for d in -(n as i32) + 1..n as i32 { let mut black_v = vec![]; for i in 0..n { let j = i as i32 + d; if j >= 0 && j < n as i32 { black_v.push(grid[i][j as usize]); } } if d <= 0 { black_v.sort_unstable_by(|a, b| b.cmp(a)); } else { black_v.sort_unstable(); } let mut it = black_v.into_iter(); for i in 0..n { let j = i as i32 + d; if j >= 0 && j < n as i32 { grid[i][j as usize] = it.next().unwrap(); } } } grid } }
```