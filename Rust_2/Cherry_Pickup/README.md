# Cherry Pickup

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given an <code>n x n</code> <code>grid</code> representing a field of cherries, each cell is one of three possible integers.</p>

<ul>
	<li><code>0</code> means the cell is empty, so you can pass through,</li>
	<li><code>1</code> means the cell contains a cherry that you can pick up and pass through, or</li>
	<li><code>-1</code> means the cell contains a thorn that blocks your way.</li>
</ul>

<p>Return <em>the maximum number of cherries you can collect by following the rules below</em>:</p>

<ul>
	<li>Starting at the position <code>(0, 0)</code> and reaching <code>(n - 1, n - 1)</code> by moving right or down through valid path cells (cells with value <code>0</code> or <code>1</code>).</li>
	<li>After reaching <code>(n - 1, n - 1)</code>, returning to <code>(0, 0)</code> by moving left or up through valid path cells.</li>
	<li>When passing through a path cell containing a cherry, you pick it up, and the cell becomes an empty cell <code>0</code>.</li>
	<li>If there is no valid path between <code>(0, 0)</code> and <code>(n - 1, n - 1)</code>, then no cherries can be collected.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/grid.jpg" style="width: 242px; height: 242px;" />
<pre>
<strong>Input:</strong> grid = [[0,1,-1],[1,0,-1],[1,1,1]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The player started at (0, 0) and went down, down, right right to reach (2, 2).
4 cherries were picked up during this single trip, and the matrix becomes [[0,1,-1],[0,0,-1],[0,0,0]].
Then, the player went left, up, up, left to return home, picking up one more cherry.
The total number of cherries picked up is 5, and this is the maximum possible.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1,-1],[1,-1,1],[-1,1,1]]
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= n &lt;= 50</code></li>
	<li><code>grid[i][j]</code> is <code>-1</code>, <code>0</code>, or <code>1</code>.</li>
	<li><code>grid[0][0] != -1</code></li>
	<li><code>grid[n - 1][n - 1] != -1</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![i32::MIN; n]; n];
        dp[0][0] = grid[0][0];

        for t in 1..2*n-1 {
            let mut ndp = vec![vec![i32::MIN; n]; n];
            for r1 in 0..n {
                let c1 = t as i32 - r1 as i32;
                if c1 < 0 || c1 >= n as i32 { continue; }
                let c1 = c1 as usize;
                if grid[r1][c1] == -1 { continue; }

                for r2 in r1..n {
                    let c2 = t as i32 - r2 as i32;
                    if c2 < 0 || c2 >= n as i32 { continue; }
                    let c2 = c2 as usize;
                    if grid[r2][c2] == -1 { continue; }

                    let mut best = i32::MIN;
                    for (pr1, pr2) in [(r1,r2),(r1+1,r2),(r1,r2+1),(r1+1,r2+1)] {
                        if pr1 > 0 && pr2 > 0 && dp[pr1-1][pr2-1] != i32::MIN {
                            best = best.max(dp[pr1-1][pr2-1]);
                        } else if pr1 == 0 && pr2 == 0 && dp[0][0] != i32::MIN && t == 1 {
                            best = best.max(dp[0][0]);
                        }
                    }

                    if best == i32::MIN { continue; }
                    let cherries = grid[r1][c1] + if r1 == r2 { 0 } else { grid[r2][c2] };
                    ndp[r1][r2] = ndp[r1][r2].max(best + cherries);
                }
            }
            dp = ndp;
        }

        dp[n-1][n-1].max(0)
    }
}
```