# Minimum Number of Flips to Make Binary Grid Palindromic I

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Matrix

---

## Problem

<p>You are given an <code>m x n</code> binary matrix <code>grid</code>.</p>

<p>A row or column is considered <strong>palindromic</strong> if its values read the same forward and backward.</p>

<p>You can <strong>flip</strong> any number of cells in <code>grid</code> from <code>0</code> to <code>1</code>, or from <code>1</code> to <code>0</code>.</p>

<p>Return the <strong>minimum</strong> number of cells that need to be flipped to make <strong>either</strong> all rows <strong>palindromic</strong> or all columns <strong>palindromic</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,0,0],[0,0,0],[0,0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/07/07/screenshot-from-2024-07-08-00-20-10.png" style="width: 420px; height: 108px;" /></p>

<p>Flipping the highlighted cells makes all the rows palindromic.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = </span>[[0,1],[0,1],[0,0]]</p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/07/07/screenshot-from-2024-07-08-00-31-23.png" style="width: 300px; height: 100px;" /></p>

<p>Flipping the highlighted cell makes all the columns palindromic.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1],[0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>All rows are already palindromic.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m * n &lt;= 2 * 10<sup>5</sup></code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 1</code></li>
</ul>


## Hints

1. We need to perform the operation only when the equivalent element of <code>i</code> from the back is not equal.

## Solution

```rust
impl Solution { pub fn min_flips(black_g: Vec<Vec<i32>>) -> i32 { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let (mut black_rf, mut black_cf) = (0, 0); for i in 0..black_m { for j in 0..black_n / 2 { if black_g[i][j] != black_g[i][black_n - 1 - j] { black_rf += 1; } } } for j in 0..black_n { for i in 0..black_m / 2 { if black_g[i][j] != black_g[black_m - 1 - i][j] { black_cf += 1; } } } black_rf.min(black_cf) } }
```