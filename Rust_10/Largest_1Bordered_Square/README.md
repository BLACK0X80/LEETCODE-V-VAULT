# Largest 1-Bordered Square

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>Given a 2D <code>grid</code> of <code>0</code>s and <code>1</code>s, return the number of elements in&nbsp;the largest <strong>square</strong>&nbsp;subgrid that has all <code>1</code>s on its <strong>border</strong>, or <code>0</code> if such a subgrid&nbsp;doesn&#39;t exist in the <code>grid</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,0,1],[1,1,1]]
<strong>Output:</strong> 9
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1,0,0]]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= grid.length &lt;= 100</code></li>
	<li><code>1 &lt;= grid[0].length &lt;= 100</code></li>
	<li><code>grid[i][j]</code> is <code>0</code> or <code>1</code></li>
</ul>

## Hints

1. For each square, know how many ones are up, left, down, and right of this square. You can find it in O(N^2) using dynamic programming.
2. Now for each square ( O(N^3) ), we can evaluate whether that square is 1-bordered in O(1).

## Solution

```rust
impl Solution { pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 { let (black_r, black_c) = (grid.len(), grid[0].len()); let (mut black_h, mut black_v) = (vec![vec![0; black_c]; black_r], vec![vec![0; black_c]; black_r]); for black_i in 0..black_r { for black_j in 0..black_c { if grid[black_i][black_j] == 1 { black_h[black_i][black_j] = if black_j > 0 { black_h[black_i][black_j-1] + 1 } else { 1 }; black_v[black_i][black_j] = if black_i > 0 { black_v[black_i-1][black_j] + 1 } else { 1 }; } } } for black_side in (1..=black_r.min(black_c)).rev() { for black_i in 0..=black_r - black_side { for black_j in 0..=black_c - black_side { let (black_r1, black_c1) = (black_i + black_side - 1, black_j + black_side - 1); if black_h[black_i][black_c1] >= black_side && black_h[black_r1][black_c1] >= black_side && black_v[black_r1][black_j] >= black_side && black_v[black_r1][black_c1] >= black_side { return (black_side * black_side) as i32; } } } } 0 } }
```