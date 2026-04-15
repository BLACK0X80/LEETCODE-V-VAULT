# Surrounded Regions

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>You are given an <code>m x n</code> matrix <code>board</code> containing <strong>letters</strong> <code>&#39;X&#39;</code> and <code>&#39;O&#39;</code>, <strong>capture regions</strong> that are <strong>surrounded</strong>:</p>

<ul>
	<li><strong>Connect</strong>: A cell is connected to adjacent cells horizontally or vertically.</li>
	<li><strong>Region</strong>: To form a region <strong>connect every</strong> <code>&#39;O&#39;</code> cell.</li>
	<li><strong>Surround</strong>: A region is surrounded if none of the <code>&#39;O&#39;</code> cells in that region are on the edge of the board. Such regions are <strong>completely enclosed </strong>by <code>&#39;X&#39;</code> cells.</li>
</ul>

<p>To capture a <strong>surrounded region</strong>, replace all <code>&#39;O&#39;</code>s with <code>&#39;X&#39;</code>s <strong>in-place</strong> within the original board. You do not need to return anything.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">board = [[&quot;X&quot;,&quot;X&quot;,&quot;X&quot;,&quot;X&quot;],[&quot;X&quot;,&quot;O&quot;,&quot;O&quot;,&quot;X&quot;],[&quot;X&quot;,&quot;X&quot;,&quot;O&quot;,&quot;X&quot;],[&quot;X&quot;,&quot;O&quot;,&quot;X&quot;,&quot;X&quot;]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[&quot;X&quot;,&quot;X&quot;,&quot;X&quot;,&quot;X&quot;],[&quot;X&quot;,&quot;X&quot;,&quot;X&quot;,&quot;X&quot;],[&quot;X&quot;,&quot;X&quot;,&quot;X&quot;,&quot;X&quot;],[&quot;X&quot;,&quot;O&quot;,&quot;X&quot;,&quot;X&quot;]]</span></p>

<p><strong>Explanation:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg" style="width: 367px; height: 158px;" />
<p>In the above diagram, the bottom region is not captured because it is on the edge of the board and cannot be surrounded.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">board = [[&quot;X&quot;]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[[&quot;X&quot;]]</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == board.length</code></li>
	<li><code>n == board[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 200</code></li>
	<li><code>board[i][j]</code> is <code>&#39;X&#39;</code> or <code>&#39;O&#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn solve(black_b: &mut Vec<Vec<char>>) { let (black_r, black_c) = (black_b.len(), black_b[0].len()); if black_r < 3 || black_c < 3 { return; } fn black_dfs(black_i: usize, black_j: usize, black_b: &mut Vec<Vec<char>>) { if black_b[black_i][black_j] != 'O' { return; } black_b[black_i][black_j] = 'T'; if black_i > 0 { black_dfs(black_i - 1, black_j, black_b); } if black_i < black_b.len() - 1 { black_dfs(black_i + 1, black_j, black_b); } if black_j > 0 { black_dfs(black_i, black_j - 1, black_b); } if black_j < black_b[0].len() - 1 { black_dfs(black_i, black_j + 1, black_b); } } for black_i in 0..black_r { black_dfs(black_i, 0, black_b); black_dfs(black_i, black_c - 1, black_b); } for black_j in 0..black_c { black_dfs(0, black_j, black_b); black_dfs(black_r - 1, black_j, black_b); } for black_row in black_b.iter_mut() { for black_val in black_row.iter_mut() { *black_val = if *black_val == 'T' { 'O' } else { 'X' }; } } } }
```