# Count Submatrices With Equal Frequency of X and Y

**Difficulty:** Medium
**Tags:** Array, Matrix, Prefix Sum

---

## Problem

<p>Given a 2D character matrix <code>grid</code>, where <code>grid[i][j]</code> is either <code>&#39;X&#39;</code>, <code>&#39;Y&#39;</code>, or <code>&#39;.&#39;</code>, return the number of <span data-keyword="submatrix">submatrices</span> that contain:</p>

<ul>
	<li><code>grid[0][0]</code></li>
	<li>an <strong>equal</strong> frequency of <code>&#39;X&#39;</code> and <code>&#39;Y&#39;</code>.</li>
	<li><strong>at least</strong> one <code>&#39;X&#39;</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[&quot;X&quot;,&quot;Y&quot;,&quot;.&quot;],[&quot;Y&quot;,&quot;.&quot;,&quot;.&quot;]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2024/06/07/examplems.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 175px; height: 350px;" /></strong></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[&quot;X&quot;,&quot;X&quot;],[&quot;X&quot;,&quot;Y&quot;]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>No submatrix has an equal frequency of <code>&#39;X&#39;</code> and <code>&#39;Y&#39;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[&quot;.&quot;,&quot;.&quot;],[&quot;.&quot;,&quot;.&quot;]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>No submatrix has at least one <code>&#39;X&#39;</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= grid.length, grid[i].length &lt;= 1000</code></li>
	<li><code>grid[i][j]</code> is either <code>&#39;X&#39;</code>, <code>&#39;Y&#39;</code>, or <code>&#39;.&#39;</code>.</li>
</ul>


## Hints

1. Replace <code>’X’</code> with 1, <code>’Y’</code> with -1 and <code>’.’</code> with 0.
2. You need to find how many submatrices <code>grid[0..x][0..y]</code> have a sum of 0 and at least one <code>’X’</code>.
3. Use prefix sum to calculate submatrices sum.

## Solution

```rust
impl Solution { pub fn number_of_submatrices(black_g: Vec<Vec<char>>) -> i32 { let (black_m, black_n) = (black_g.len(), black_g[0].len()); let (mut black_x, mut black_y) = (vec![vec![0; black_n + 1]; black_m + 1], vec![vec![0; black_n + 1]; black_m + 1]); let mut black_res = 0; for i in 0..black_m { for j in 0..black_n { black_x[i+1][j+1] = black_x[i][j+1] + black_x[i+1][j] - black_x[i][j] + (if black_g[i][j] == 'X' {1} else {0}); black_y[i+1][j+1] = black_y[i][j+1] + black_y[i+1][j] - black_y[i][j] + (if black_g[i][j] == 'Y' {1} else {0}); if black_x[i+1][j+1] > 0 && black_x[i+1][j+1] == black_y[i+1][j+1] { black_res += 1; } } } black_res } }
```