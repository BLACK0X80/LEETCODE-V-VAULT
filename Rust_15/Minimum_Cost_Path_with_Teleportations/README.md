# Minimum Cost Path with Teleportations

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given a <code>m x n</code> 2D integer array <code>grid</code> and an integer <code>k</code>. You start at the top-left cell <code>(0, 0)</code> and your goal is to reach the bottom‐right cell <code>(m - 1, n - 1)</code>.</p>

<p>There are two types of moves available:</p>

<ul>
	<li>
	<p><strong>Normal move</strong>: You can move right or down from your current cell <code>(i, j)</code>, i.e. you can move to <code>(i, j + 1)</code> (right) or <code>(i + 1, j)</code> (down). The cost is the value of the destination cell.</p>
	</li>
	<li>
	<p><strong>Teleportation</strong>: You can teleport from any cell <code>(i, j)</code>, to any cell <code>(x, y)</code> such that <code>grid[x][y] &lt;= grid[i][j]</code>; the cost of this move is 0. You may teleport at most <code>k</code> times.</p>
	</li>
</ul>

<p>Return the <strong>minimum</strong> total cost to reach cell <code>(m - 1, n - 1)</code> from <code>(0, 0)</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,3,3],[2,5,4],[4,3,5]], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<p>Initially we are at (0, 0) and cost is 0.</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr>
			<th style="border: 1px solid black;">Current Position</th>
			<th style="border: 1px solid black;">Move</th>
			<th style="border: 1px solid black;">New Position</th>
			<th style="border: 1px solid black;">Total Cost</th>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>(0, 0)</code></td>
			<td style="border: 1px solid black;">Move Down</td>
			<td style="border: 1px solid black;"><code>(1, 0)</code></td>
			<td style="border: 1px solid black;"><code>0 + 2 = 2</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>(1, 0)</code></td>
			<td style="border: 1px solid black;">Move Right</td>
			<td style="border: 1px solid black;"><code>(1, 1)</code></td>
			<td style="border: 1px solid black;"><code>2 + 5 = 7</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>(1, 1)</code></td>
			<td style="border: 1px solid black;">Teleport to <code>(2, 2)</code></td>
			<td style="border: 1px solid black;"><code>(2, 2)</code></td>
			<td style="border: 1px solid black;"><code>7 + 0 = 7</code></td>
		</tr>
	</tbody>
</table>

<p>The minimum cost to reach bottom-right cell is 7.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,2],[2,3],[3,4]], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation: </strong></p>

<p>Initially we are at (0, 0) and cost is 0.</p>

<table style="border: 1px solid black;">
	<tbody>
		<tr>
			<th style="border: 1px solid black;">Current Position</th>
			<th style="border: 1px solid black;">Move</th>
			<th style="border: 1px solid black;">New Position</th>
			<th style="border: 1px solid black;">Total Cost</th>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>(0, 0)</code></td>
			<td style="border: 1px solid black;">Move Down</td>
			<td style="border: 1px solid black;"><code>(1, 0)</code></td>
			<td style="border: 1px solid black;"><code>0 + 2 = 2</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>(1, 0)</code></td>
			<td style="border: 1px solid black;">Move Right</td>
			<td style="border: 1px solid black;"><code>(1, 1)</code></td>
			<td style="border: 1px solid black;"><code>2 + 3 = 5</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;"><code>(1, 1)</code></td>
			<td style="border: 1px solid black;">Move Down</td>
			<td style="border: 1px solid black;"><code>(2, 1)</code></td>
			<td style="border: 1px solid black;"><code>5 + 4 = 9</code></td>
		</tr>
	</tbody>
</table>

<p>The minimum cost to reach bottom-right cell is 9.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= m, n &lt;= 80</code></li>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= k &lt;= 10</code></li>
</ul>


## Hints

1. Use dynamic programming to solve the problem efficiently.
2. Think of the solution in terms of up to <code>k</code> teleportation steps. At each step, compute the minimum cost to reach each cell, either through a normal move or a teleportation from the previous step.

## Solution

```rust
impl Solution {
    pub fn min_cost(black_grid: Vec<Vec<i32>>, black_k: i32) -> i32 {
        let black_n = black_grid.len();
        let black_m = black_grid[0].len();
        
        let mut black_max_val = 0;
        for black_row in &black_grid {
            for &black_val in black_row {
                black_max_val = black_max_val.max(black_val);
            }
        }

        let black_max_v = black_max_val as usize;
        let mut black_dp = vec![vec![i32::MAX; black_m]; black_n];
        let mut black_temp = vec![i32::MAX; black_max_v + 1];
        let mut black_best = vec![i32::MAX; black_max_v + 1];

        black_dp[black_n - 1][black_m - 1] = 0;
        black_temp[black_grid[black_n - 1][black_m - 1] as usize] = 0;

        for black_i in (0..black_n).rev() {
            for black_j in (0..black_m).rev() {
                if black_i == black_n - 1 && black_j == black_m - 1 { continue; }
                
                let mut black_res = i32::MAX;
                if black_i + 1 < black_n && black_dp[black_i + 1][black_j] != i32::MAX {
                    black_res = black_res.min(black_dp[black_i + 1][black_j] + black_grid[black_i + 1][black_j]);
                }
                if black_j + 1 < black_m && black_dp[black_i][black_j + 1] != i32::MAX {
                    black_res = black_res.min(black_dp[black_i][black_j + 1] + black_grid[black_i][black_j + 1]);
                }

                black_dp[black_i][black_j] = black_res;
                
                let black_v_idx = black_grid[black_i][black_j] as usize;
                if black_dp[black_i][black_j] != i32::MAX {
                    black_temp[black_v_idx] = black_temp[black_v_idx].min(black_dp[black_i][black_j]);
                }
            }
        }

        for _ in 0..black_k {
            black_best[0] = black_temp[0];
            for black_v in 1..=black_max_v {
                black_best[black_v] = black_best[black_v - 1].min(black_temp[black_v]);
            }
            
            for black_i in (0..black_n).rev() {
                for black_j in (0..black_m).rev() {
                    if black_i == black_n - 1 && black_j == black_m - 1 { continue; }
                    
                    let mut black_walk = i32::MAX;
                    if black_i + 1 < black_n && black_dp[black_i + 1][black_j] != i32::MAX {
                        black_walk = black_walk.min(black_dp[black_i + 1][black_j] + black_grid[black_i + 1][black_j]);
                    }
                    if black_j + 1 < black_m && black_dp[black_i][black_j + 1] != i32::MAX {
                        black_walk = black_walk.min(black_dp[black_i][black_j + 1] + black_grid[black_i][black_j + 1]);
                    }

                    let black_v_idx = black_grid[black_i][black_j] as usize;
                    let black_teleport = black_best[black_v_idx];
                    
                    black_dp[black_i][black_j] = black_walk.min(black_teleport);
                    
                    if black_dp[black_i][black_j] != i32::MAX {
                        black_temp[black_v_idx] = black_temp[black_v_idx].min(black_dp[black_i][black_j]);
                    }
                }
            }
        }
        
        black_dp[0][0]
    }
}
```