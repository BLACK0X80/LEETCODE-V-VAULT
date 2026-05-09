# Largest Magic Square

**Difficulty:** Medium
**Tags:** Array, Matrix, Prefix Sum

---

## Problem

<p>A <code>k x k</code> <strong>magic square</strong> is a <code>k x k</code> grid filled with integers such that every row sum, every column sum, and both diagonal sums are <strong>all equal</strong>. The integers in the magic square <strong>do not have to be distinct</strong>. Every <code>1 x 1</code> grid is trivially a <strong>magic square</strong>.</p>

<p>Given an <code>m x n</code> integer <code>grid</code>, return <em>the <strong>size</strong> (i.e., the side length </em><code>k</code><em>) of the <strong>largest magic square</strong> that can be found within this grid</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/29/magicsquare-grid.jpg" style="width: 413px; height: 335px;" />
<pre>
<strong>Input:</strong> grid = [[7,1,4,5,6],[2,5,1,6,4],[1,5,4,3,2],[1,2,7,3,4]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The largest magic square has a size of 3.
Every row sum, column sum, and diagonal sum of this magic square is equal to 12.
- Row sums: 5+1+6 = 5+4+3 = 2+7+3 = 12
- Column sums: 5+5+2 = 1+4+7 = 6+3+3 = 12
- Diagonal sums: 5+4+3 = 6+4+2 = 12
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/29/magicsquare2-grid.jpg" style="width: 333px; height: 255px;" />
<pre>
<strong>Input:</strong> grid = [[5,1,3,1],[9,3,3,1],[1,3,3,8]]
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 50</code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Check all squares in the matrix and find the largest one.

## Solution

```rust
impl Solution { pub fn largest_magic_square(black_g: Vec<Vec<i32>>) -> i32 { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let (mut black_rs, mut black_cs) = (vec![vec![0; black_n + 1]; black_m], vec![vec![0; black_n]; black_m + 1]); for i in 0..black_m { for j in 0..black_n { black_rs[i][j+1] = black_rs[i][j] + black_g[i][j]; black_cs[i+1][j] = black_cs[i][j] + black_g[i][j]; } } for black_k in (2..=black_m.min(black_n)).rev() { for i in 0..=black_m - black_k { for j in 0..=black_n - black_k { let black_target = black_rs[i][j+black_k] - black_rs[i][j]; if (0..black_k).all(|r| black_rs[i+r][j+black_k] - black_rs[i+r][j] == black_target) && (0..black_k).all(|c| black_cs[i+black_k][j+c] - black_cs[i][j+c] == black_target) { let (mut black_d1, mut black_d2) = (0, 0); for d in 0..black_k { black_d1 += black_g[i+d][j+d]; black_d2 += black_g[i+d][j+black_k-1-d]; } if black_d1 == black_target && black_d2 == black_target { return black_k as i32; } } } } } 1 } }
```