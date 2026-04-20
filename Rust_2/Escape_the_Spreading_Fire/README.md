# Escape the Spreading Fire

**Difficulty:** Hard
**Tags:** Array, Binary Search, Breadth-First Search, Matrix

---

## Problem

<p>You are given a <strong>0-indexed</strong> 2D integer array <code>grid</code> of size <code>m x n</code> which represents a field. Each cell has one of three values:</p>

<ul>
	<li><code>0</code> represents grass,</li>
	<li><code>1</code> represents fire,</li>
	<li><code>2</code> represents a wall that you and fire cannot pass through.</li>
</ul>

<p>You are situated in the top-left cell, <code>(0, 0)</code>, and you want to travel to the safehouse at the bottom-right cell, <code>(m - 1, n - 1)</code>. Every minute, you may move to an <strong>adjacent</strong> grass cell. <strong>After</strong> your move, every fire cell will spread to all <strong>adjacent</strong> cells that are not walls.</p>

<p>Return <em>the <strong>maximum</strong> number of minutes that you can stay in your initial position before moving while still safely reaching the safehouse</em>. If this is impossible, return <code>-1</code>. If you can <strong>always</strong> reach the safehouse regardless of the minutes stayed, return <code>10<sup>9</sup></code>.</p>

<p>Note that even if the fire spreads to the safehouse immediately after you have reached it, it will be counted as safely reaching the safehouse.</p>

<p>A cell is <strong>adjacent</strong> to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/10/ex1new.jpg" style="width: 650px; height: 404px;" />
<pre>
<strong>Input:</strong> grid = [[0,2,0,0,0,0,0],[0,0,0,2,2,1,0],[0,2,0,0,1,2,0],[0,0,2,2,2,0,2],[0,0,0,0,0,0,0]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The figure above shows the scenario where you stay in the initial position for 3 minutes.
You will still be able to safely reach the safehouse.
Staying for more than 3 minutes will not allow you to safely reach the safehouse.</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/10/ex2new2.jpg" style="width: 515px; height: 150px;" />
<pre>
<strong>Input:</strong> grid = [[0,0,0,0],[0,1,2,0],[0,2,0,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The figure above shows the scenario where you immediately move towards the safehouse.
Fire will spread to any cell you move towards and it is impossible to safely reach the safehouse.
Thus, -1 is returned.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/10/ex3new.jpg" style="width: 174px; height: 150px;" />
<pre>
<strong>Input:</strong> grid = [[0,0,0],[2,2,0],[1,2,0]]
<strong>Output:</strong> 1000000000
<strong>Explanation:</strong> The figure above shows the initial grid.
Notice that the fire is contained by walls and you will always be able to safely reach the safehouse.
Thus, 10<sup>9</sup> is returned.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>2 &lt;= m, n &lt;= 300</code></li>
	<li><code>4 &lt;= m * n &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>grid[i][j]</code> is either <code>0</code>, <code>1</code>, or <code>2</code>.</li>
	<li><code>grid[0][0] == grid[m - 1][n - 1] == 0</code></li>
</ul>


## Hints

1. For some tile (x, y), how can we determine when, if ever, the fire will reach it?
2. We can use multi-source BFS to find the earliest time the fire will reach each cell.
3. Then, starting with a given t minutes of staying in the initial position, we can check if there is a safe path to the safehouse using the obtained information about the fire.
4. We can use binary search to efficiently find the maximum t that allows us to reach the safehouse.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut fire = vec![vec![i32::MAX; n]; m];
        let mut q = VecDeque::new();
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    fire[r][c] = 0;
                    q.push_back((r, c));
                }
            }
        }
        let dirs = [(0,1),(0,-1),(1,0),(-1,0)];
        while let Some((r, c)) = q.pop_front() {
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if grid[nr][nc] != 2 && fire[nr][nc] == i32::MAX {
                        fire[nr][nc] = fire[r][c] + 1;
                        q.push_back((nr, nc));
                    }
                }
            }
        }
        let check = |w: i32| -> bool {
            let mut vis = vec![vec![false; n]; m];
            let mut q = VecDeque::new();
            if fire[0][0] > w {
                q.push_back((0, 0, w));
                vis[0][0] = true;
            }
            while let Some((r, c, t)) = q.pop_front() {
                for (dr, dc) in dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        if !vis[nr][nc] && grid[nr][nc] != 2 {
                            let nt = t + 1;
                            if (nr == m - 1 && nc == n - 1 && nt <= fire[nr][nc]) || nt < fire[nr][nc] {
                                if nr == m - 1 && nc == n - 1 { return true; }
                                vis[nr][nc] = true;
                                q.push_back((nr, nc, nt));
                            }
                        }
                    }
                }
            }
            false
        };
        if !check(0) { return -1; }
        if check(1_000_000_000) { return 1_000_000_000; }
        let (mut l, mut r) = (0, 1_000_000_000);
        while l < r {
            let mid = l + (r - l + 1) / 2;
            if check(mid) { l = mid; } else { r = mid - 1; }
        }
        l
    }
}
```