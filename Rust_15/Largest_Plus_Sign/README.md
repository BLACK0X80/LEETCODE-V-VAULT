# Largest Plus Sign

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming

---

## Problem

<p>You are given an integer <code>n</code>. You have an <code>n x n</code> binary grid <code>grid</code> with all values initially <code>1</code>&#39;s except for some indices given in the array <code>mines</code>. The <code>i<sup>th</sup></code> element of the array <code>mines</code> is defined as <code>mines[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> where <code>grid[x<sub>i</sub>][y<sub>i</sub>] == 0</code>.</p>

<p>Return <em>the order of the largest <strong>axis-aligned</strong> plus sign of </em>1<em>&#39;s contained in </em><code>grid</code>. If there is none, return <code>0</code>.</p>

<p>An <strong>axis-aligned plus sign</strong> of <code>1</code>&#39;s of order <code>k</code> has some center <code>grid[r][c] == 1</code> along with four arms of length <code>k - 1</code> going up, down, left, and right, and made of <code>1</code>&#39;s. Note that there could be <code>0</code>&#39;s or <code>1</code>&#39;s beyond the arms of the plus sign, only the relevant area of the plus sign is checked for <code>1</code>&#39;s.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/plus1-grid.jpg" style="width: 404px; height: 405px;" />
<pre>
<strong>Input:</strong> n = 5, mines = [[4,2]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> In the above grid, the largest plus sign can only be of order 2. One of them is shown.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/plus2-grid.jpg" style="width: 84px; height: 85px;" />
<pre>
<strong>Input:</strong> n = 1, mines = [[0,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no plus sign, so return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 500</code></li>
	<li><code>1 &lt;= mines.length &lt;= 5000</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt; n</code></li>
	<li>All the pairs <code>(x<sub>i</sub>, y<sub>i</sub>)</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. For each direction such as "left", find left[r][c] = the number of 1s you will see before a zero starting at r, c and walking left.  You can find this in N^2 time with a dp.  The largest plus sign at r, c is just the minimum of left[r][c], up[r][c] etc.

## Solution

```rust
impl Solution { pub fn order_of_largest_plus_sign(black_n: i32, black_m: Vec<Vec<i32>>) -> i32 { let n = black_n as usize; let mut black_grid = vec![vec![black_n; n]; n]; for m in black_m { black_grid[m[0] as usize][m[1] as usize] = 0; } for i in 0..n { let (mut l, mut r, mut u, mut d) = (0, 0, 0, 0); for j in 0..n { l = if black_grid[i][j] == 0 { 0 } else { l + 1 }; black_grid[i][j] = black_grid[i][j].min(l); let k = n - 1 - j; r = if black_grid[i][k] == 0 { 0 } else { r + 1 }; black_grid[i][k] = black_grid[i][k].min(r); u = if black_grid[j][i] == 0 { 0 } else { u + 1 }; black_grid[j][i] = black_grid[j][i].min(u); let k2 = n - 1 - j; d = if black_grid[k2][i] == 0 { 0 } else { d + 1 }; black_grid[k2][i] = black_grid[k2][i].min(d); } } black_grid.into_iter().flat_map(|r| r.into_iter()).max().unwrap_or(0) } }
```