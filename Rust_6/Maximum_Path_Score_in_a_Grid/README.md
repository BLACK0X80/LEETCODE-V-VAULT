# Maximum Path Score in a Grid

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given an <code>m x n</code> grid where each cell contains one of the values 0, 1, or 2. You are also given an integer <code>k</code>.</p>

<p>You start from the top-left corner <code>(0, 0)</code> and want to reach the bottom-right corner <code>(m - 1, n - 1)</code> by moving only <strong>right</strong> or <strong>down</strong>.</p>

<p>Each cell contributes a specific score and incurs an associated cost, according to their cell values:</p>

<ul>
	<li>0: adds 0 to your score and costs 0.</li>
	<li>1: adds 1 to your score and costs 1.</li>
	<li>2: adds 2 to your score and costs 1. ​​​​​​​</li>
</ul>

<p>Return the <strong>maximum</strong> score achievable without exceeding a total cost of <code>k</code>, or -1 if no valid path exists.</p>

<p><strong>Note:</strong> If you reach the last cell but the total cost exceeds <code>k</code>, the path is invalid.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0, 1],[2, 0]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong>​​​​​​​</p>

<p>The optimal path is:</p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;">Cell</th>
			<th style="border: 1px solid black;">grid[i][j]</th>
			<th style="border: 1px solid black;">Score</th>
			<th style="border: 1px solid black;">Total<br />
			Score</th>
			<th style="border: 1px solid black;">Cost</th>
			<th style="border: 1px solid black;">Total<br />
			Cost</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">(0, 0)</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">(1, 0)</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">(1, 1)</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;">1</td>
		</tr>
	</tbody>
</table>

<p>Thus, the maximum possible score is 2.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0, 1],[1, 2]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>There is no path that reaches cell <code>(1, 1)</code>​​​​​​​ without exceeding cost k. Thus, the answer is -1.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m, n &lt;= 200</code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>3</sup>​​​​​​​</code></li>
	<li><code><sup>​​​​​​​</sup>grid[0][0] == 0</code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 2</code></li>
</ul>


## Hints

1. Use dynamic programming.
2. Let <code>dp[i][j][c]</code> = max score at cell <code>(i,j)</code> with total cost exactly <code>c</code> (0 <= <code>c</code> <= <code>k</code>).
3. Update <code>dp[i][j][c]</code> from <code>(i-1,j)</code> and <code>(i,j-1)</code> using <code>cost = (grid[i][j] == 0 ? 0 : 1)</code> and <code>score = grid[i][j]</code>.
4. Answer = <code>max(dp[m-1][n-1][c])</code> for <code>c=0..k</code>, or <code>-1</code> if none.

## Solution

```rust
impl Solution { pub fn max_path_score(black_grid: Vec<Vec<i32>>, black_k: i32) -> i32 { let (black_m, black_n, black_k) = (black_grid.len(), black_grid[0].len(), black_k as usize); let mut black_dp = vec![vec![vec![-1; black_k + 1]; black_n]; black_m]; black_dp[0][0][0] = 0; for black_i in 0..black_m { for black_j in 0..black_n { for black_c in 0..=black_k { if black_dp[black_i][black_j][black_c] == -1 { continue; } for (black_di, black_dj) in [(0, 1), (1, 0)] { let (black_ni, black_nj) = (black_i + black_di, black_j + black_dj); if black_ni < black_m && black_nj < black_n { let black_val = black_grid[black_ni][black_nj]; let (black_s, black_cost) = (if black_val == 1 {1} else if black_val == 2 {2} else {0}, if black_val == 0 {0} else {1}); if black_c + black_cost <= black_k { black_dp[black_ni][black_nj][black_c + black_cost] = black_dp[black_ni][black_nj][black_c + black_cost].max(black_dp[black_i][black_j][black_c] + black_s); } } } } } } black_dp[black_m - 1][black_n - 1].iter().max().cloned().unwrap_or(-1).max(-1) } }
```