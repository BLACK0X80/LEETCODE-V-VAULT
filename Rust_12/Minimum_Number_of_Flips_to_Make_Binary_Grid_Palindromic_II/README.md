# Minimum Number of Flips to Make Binary Grid Palindromic II

**Difficulty:** Medium
**Tags:** Array, Two Pointers, Matrix

---

## Problem

<p>You are given an <code>m x n</code> binary matrix <code>grid</code>.</p>

<p>A row or column is considered <strong>palindromic</strong> if its values read the same forward and backward.</p>

<p>You can <strong>flip</strong> any number of cells in <code>grid</code> from <code>0</code> to <code>1</code>, or from <code>1</code> to <code>0</code>.</p>

<p>Return the <strong>minimum</strong> number of cells that need to be flipped to make <strong>all</strong> rows and columns <strong>palindromic</strong>, and the total number of <code>1</code>&#39;s in <code>grid</code> <strong>divisible</strong> by <code>4</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,0,0],[0,1,0],[0,0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2024/08/01/image.png" style="width: 400px; height: 105px;" /></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0,1],[0,1],[0,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/07/08/screenshot-from-2024-07-09-01-37-48.png" style="width: 300px; height: 104px;" /></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1],[1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/01/screenshot-from-2024-08-01-23-05-26.png" style="width: 200px; height: 70px;" /></p>
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

1. For each <code>(x, y)</code>, find <code>(m - 1 - x, y)</code>, <code>(m - 1 - x, n - 1 - y)</code>, and <code>(x, n - 1 - y)</code>; they should be the same.
2. Note that we need to specially handle the middle row (column) if the number of rows (columns) is odd.

## Solution

```rust
impl Solution { pub fn min_flips(black_g: Vec<Vec<i32>>) -> i32 { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let (mut black_f, mut black_o) = (0, 0); for i in 0..black_m / 2 { for j in 0..black_n / 2 { let black_v = [black_g[i][j], black_g[i][black_n-1-j], black_g[black_m-1-i][j], black_g[black_m-1-i][black_n-1-j]]; let black_c = black_v.iter().filter(|&&x| x == 1).count(); black_f += black_c.min(4 - black_c) as i32; } } let (mut black_p, mut black_s) = (0, 0); if black_m % 2 == 1 { let r = black_m / 2; for j in 0..black_n / 2 { if black_g[r][j] != black_g[r][black_n-1-j] { black_p += 1; } else if black_g[r][j] == 1 { black_s += 2; } } } if black_n % 2 == 1 { let c = black_n / 2; for i in 0..black_m / 2 { if black_g[i][c] != black_g[black_m-1-i][c] { black_p += 1; } else if black_g[i][c] == 1 { black_s += 2; } } } if black_m % 2 == 1 && black_n % 2 == 1 && black_g[black_m/2][black_n/2] == 1 { black_f += 1; } if black_p > 0 { black_f += black_p; } else { black_f += black_s % 4; } black_f } }
```