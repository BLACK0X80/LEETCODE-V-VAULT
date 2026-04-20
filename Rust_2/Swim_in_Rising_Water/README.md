# Swim in Rising Water

**Difficulty:** Hard
**Tags:** Array, Binary Search, Depth-First Search, Breadth-First Search, Union-Find, Heap (Priority Queue), Matrix

---

## Problem

<p>You are given an <code>n x n</code> integer matrix <code>grid</code> where each value <code>grid[i][j]</code> represents the elevation at that point <code>(i, j)</code>.</p>

<p>It starts raining, and water gradually rises over time. At time <code>t</code>, the water level is <code>t</code>, meaning <strong>any</strong> cell with elevation less than equal to <code>t</code> is submerged or reachable.</p>

<p>You can swim from a square to another 4-directionally adjacent square if and only if the elevation of both squares individually are at most <code>t</code>. You can swim infinite distances in zero time. Of course, you must stay within the boundaries of the grid during your swim.</p>

<p>Return <em>the minimum time until you can reach the bottom right square </em><code>(n - 1, n - 1)</code><em> if you start at the top left square </em><code>(0, 0)</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/swim1-grid.jpg" style="width: 164px; height: 165px;" />
<pre>
<strong>Input:</strong> grid = [[0,2],[1,3]]
<strong>Output:</strong> 3
Explanation:
At time 0, you are in grid location (0, 0).
You cannot go anywhere else because 4-directionally adjacent neighbors have a higher elevation than t = 0.
You cannot reach point (1, 1) until time 3.
When the depth of water is 3, we can swim anywhere inside the grid.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/swim2-grid-1.jpg" style="width: 404px; height: 405px;" />
<pre>
<strong>Input:</strong> grid = [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]
<strong>Output:</strong> 16
<strong>Explanation:</strong> The final route is shown.
We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= n &lt;= 50</code></li>
	<li><code>0 &lt;= grid[i][j] &lt;&nbsp;n<sup>2</sup></code></li>
	<li>Each value <code>grid[i][j]</code> is <strong>unique</strong>.</li>
</ul>


## Hints

1. Use either Dijkstra's, or binary search for the best time T for which you can reach the end if you only step on squares at most T.

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut vis = vec![vec![false; n]; n];
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((grid[0][0], 0, 0)));
        vis[0][0] = true;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some(Reverse((t, r, c))) = pq.pop() {
            if r == n - 1 && c == n - 1 { return t; }
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !vis[nr][nc] {
                        vis[nr][nc] = true;
                        pq.push(Reverse((t.max(grid[nr][nc]), nr, nc)));
                    }
                }
            }
        }
        0
    }
}
```