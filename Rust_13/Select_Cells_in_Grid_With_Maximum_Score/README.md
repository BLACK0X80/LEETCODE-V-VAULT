# Select Cells in Grid With Maximum Score

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Bit Manipulation, Matrix, Bitmask

---

## Problem

<p>You are given a 2D matrix <code>grid</code> consisting of positive integers.</p>

<p>You have to select <em>one or more</em> cells from the matrix such that the following conditions are satisfied:</p>

<ul>
	<li>No two selected cells are in the <strong>same</strong> row of the matrix.</li>
	<li>The values in the set of selected cells are <strong>unique</strong>.</li>
</ul>

<p>Your score will be the <strong>sum</strong> of the values of the selected cells.</p>

<p>Return the <strong>maximum</strong> score you can achieve.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,2,3],[4,3,2],[1,1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">8</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/07/29/grid1drawio.png" /></p>

<p>We can select the cells with values 1, 3, and 4 that are colored above.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[8,7,6],[8,3,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">15</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/07/29/grid8_8drawio.png" style="width: 170px; height: 114px;" /></p>

<p>We can select the cells with values 7 and 8 that are colored above.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= grid.length, grid[i].length &lt;= 10</code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 100</code></li>
</ul>


## Hints

1. Sort all the cells in the grid by their values and keep track of their original positions.
2. Try dynamic programming with the following states: the current cell that we might select and a bitmask representing all the rows from which we have already selected a cell so far.

## Solution

```rust
impl Solution {
    pub fn max_score(black_grid: Vec<Vec<i32>>) -> i32 {
        let black_m = black_grid.len();
        let mut black_val_to_rows = std::collections::HashMap::new();
        for black_r in 0..black_m {
            for &black_v in &black_grid[black_r] {
                black_val_to_rows.entry(black_v).or_insert(0).set_bit(black_r, true);
            }
        }

        let mut black_unique_vals: Vec<_> = black_val_to_rows.keys().cloned().collect();
        black_unique_vals.sort_unstable_by(|a, b| b.cmp(a));

        let mut black_dp = vec![-1; 1 << black_m];
        black_dp[0] = 0;

        for black_v in black_unique_vals {
            let mut black_next_dp = black_dp.clone();
            let black_rows_mask = black_val_to_rows[&black_v];
            for black_mask in 0..(1 << black_m) {
                if black_dp[black_mask] == -1 { continue; }
                for black_r in 0..black_m {
                    if (black_rows_mask & (1 << black_r)) != 0 && (black_mask & (1 << black_r)) == 0 {
                        let black_new_mask = black_mask | (1 << black_r);
                        black_next_dp[black_new_mask] = black_next_dp[black_new_mask].max(black_dp[black_mask] + black_v);
                    }
                }
            }
            black_dp = black_next_dp;
        }
        *black_dp.iter().max().unwrap()
    }
}

trait BlackBitUtil { fn set_bit(&mut self, i: usize, v: bool); }
impl BlackBitUtil for i32 {
    fn set_bit(&mut self, i: usize, v: bool) { if v { *self |= 1 << i; } else { *self &= !(1 << i); } }
}
```