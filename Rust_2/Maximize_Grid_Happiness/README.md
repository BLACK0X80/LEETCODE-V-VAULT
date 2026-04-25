# Maximize Grid Happiness

**Difficulty:** Hard
**Tags:** Dynamic Programming, Bit Manipulation, Memoization, Bitmask

---

## Problem

<p>You are given four integers, <code>m</code>, <code>n</code>, <code>introvertsCount</code>, and <code>extrovertsCount</code>. You have an <code>m x n</code> grid, and there are two types of people: introverts and extroverts. There are <code>introvertsCount</code> introverts and <code>extrovertsCount</code> extroverts.</p>

<p>You should decide how many people you want to live in the grid and assign each of them one grid cell. Note that you <strong>do not</strong> have to have all the people living in the grid.</p>

<p>The <strong>happiness</strong> of each person is calculated as follows:</p>

<ul>
	<li>Introverts <strong>start</strong> with <code>120</code> happiness and <strong>lose</strong> <code>30</code> happiness for each neighbor (introvert or extrovert).</li>
	<li>Extroverts <strong>start</strong> with <code>40</code> happiness and <strong>gain</strong> <code>20</code> happiness for each neighbor (introvert or extrovert).</li>
</ul>

<p>Neighbors live in the directly adjacent cells north, east, south, and west of a person&#39;s cell.</p>

<p>The <strong>grid happiness</strong> is the <strong>sum</strong> of each person&#39;s happiness. Return<em> the <strong>maximum possible grid happiness</strong>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/grid_happiness.png" style="width: 261px; height: 121px;" />
<pre>
<strong>Input:</strong> m = 2, n = 3, introvertsCount = 1, extrovertsCount = 2
<strong>Output:</strong> 240
<strong>Explanation:</strong> Assume the grid is 1-indexed with coordinates (row, column).
We can put the introvert in cell (1,1) and put the extroverts in cells (1,3) and (2,3).
- Introvert at (1,1) happiness: 120 (starting happiness) - (0 * 30) (0 neighbors) = 120
- Extrovert at (1,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
- Extrovert at (2,3) happiness: 40 (starting happiness) + (1 * 20) (1 neighbor) = 60
The grid happiness is 120 + 60 + 60 = 240.
The above figure shows the grid in this example with each person&#39;s happiness. The introvert stays in the light green cell while the extroverts live on the light purple cells.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> m = 3, n = 1, introvertsCount = 2, extrovertsCount = 1
<strong>Output:</strong> 260
<strong>Explanation:</strong> Place the two introverts in (1,1) and (3,1) and the extrovert at (2,1).
- Introvert at (1,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
- Extrovert at (2,1) happiness: 40 (starting happiness) + (2 * 20) (2 neighbors) = 80
- Introvert at (3,1) happiness: 120 (starting happiness) - (1 * 30) (1 neighbor) = 90
The grid happiness is 90 + 80 + 90 = 260.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> m = 2, n = 2, introvertsCount = 4, extrovertsCount = 0
<strong>Output:</strong> 240
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= m, n &lt;= 5</code></li>
	<li><code>0 &lt;= introvertsCount, extrovertsCount &lt;= min(m * n, 6)</code></li>
</ul>


## Hints

1. For each cell, it has 3 options, either it is empty, or contains an introvert, or an extrovert.
2. You can do DP where you maintain the state of the previous row, the number of remaining introverts and extroverts, the current row and column, and try the 3 options for each cell.
3. Assume that the previous columns in the current row already belong to the previous row.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let black_m = m as usize;
        let black_n = n as usize;
        let mut black_memo = HashMap::new();
        let black_pow3: Vec<usize> = (0..=black_n).map(|black_i| 3usize.pow(black_i as u32)).collect();

        fn black_dfs(black_pos: usize, black_mask: usize, black_ic: usize, black_ec: usize, black_m: usize, black_n: usize, black_pow3: &[usize], black_memo: &mut HashMap<(usize, usize, usize, usize), i32>) -> i32 {
            if black_pos == black_m * black_n || (black_ic == 0 && black_ec == 0) { return 0; }
            if let Some(&black_v) = black_memo.get(&(black_pos, black_mask, black_ic, black_ec)) { return black_v; }

            let black_col = black_pos % black_n;
            let black_up = (black_mask / black_pow3[black_n - 1]) % 3;
            let black_left = if black_col > 0 { black_mask % 3 } else { 0 };
            let black_base = (black_mask * 3) % black_pow3[black_n];

            let mut black_res = black_dfs(black_pos + 1, black_base, black_ic, black_ec, black_m, black_n, black_pow3, black_memo);

            if black_ic > 0 {
                let mut black_score = 120;
                if black_up == 1 { black_score -= 60; } else if black_up == 2 { black_score -= 10; }
                if black_left == 1 { black_score -= 60; } else if black_left == 2 { black_score -= 10; }
                black_res = black_res.max(black_score + black_dfs(black_pos + 1, black_base + 1, black_ic - 1, black_ec, black_m, black_n, black_pow3, black_memo));
            }

            if black_ec > 0 {
                let mut black_score = 40;
                if black_up == 1 { black_score -= 10; } else if black_up == 2 { black_score += 40; }
                if black_left == 1 { black_score -= 10; } else if black_left == 2 { black_score += 40; }
                black_res = black_res.max(black_score + black_dfs(black_pos + 1, black_base + 2, black_ic, black_ec - 1, black_m, black_n, black_pow3, black_memo));
            }

            black_memo.insert((black_pos, black_mask, black_ic, black_ec), black_res);
            black_res
        }

        black_dfs(0, 0, introverts_count as usize, extroverts_count as usize, black_m, black_n, &black_pow3, &mut black_memo)
    }
}
```