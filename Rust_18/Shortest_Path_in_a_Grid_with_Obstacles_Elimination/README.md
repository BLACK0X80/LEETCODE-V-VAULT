# Shortest Path in a Grid with Obstacles Elimination

**Difficulty:** Hard
**Tags:** Array, Breadth-First Search, Matrix

---

## Problem

<p>You are given an <code>m x n</code> integer matrix <code>grid</code> where each cell is either <code>0</code> (empty) or <code>1</code> (obstacle). You can move up, down, left, or right from and to an empty cell in <strong>one step</strong>.</p>

<p>Return <em>the minimum number of <strong>steps</strong> to walk from the upper left corner </em><code>(0, 0)</code><em> to the lower right corner </em><code>(m - 1, n - 1)</code><em> given that you can eliminate <strong>at most</strong> </em><code>k</code><em> obstacles</em>. If it is not possible to find such walk return <code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/09/30/short1-grid.jpg" style="width: 244px; height: 405px;" />
<pre>
<strong>Input:</strong> grid = [[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], k = 1
<strong>Output:</strong> 6
<strong>Explanation:</strong> 
The shortest path without eliminating any obstacle is 10.
The shortest path with one obstacle elimination at position (3,2) is 6. Such path is (0,0) -&gt; (0,1) -&gt; (0,2) -&gt; (1,2) -&gt; (2,2) -&gt; <strong>(3,2)</strong> -&gt; (4,2).
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/09/30/short2-grid.jpg" style="width: 244px; height: 245px;" />
<pre>
<strong>Input:</strong> grid = [[0,1,1],[1,1,1],[1,0,0]], k = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong> We need to eliminate at least two obstacles to find such a walk.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 40</code></li>
	<li><code>1 &lt;= k &lt;= m * n</code></li>
	<li><code>grid[i][j]</code> is either <code>0</code> <strong>or</strong> <code>1</code>.</li>
	<li><code>grid[0][0] == grid[m - 1][n - 1] == 0</code></li>
</ul>


## Hints

1. Use BFS.
2. BFS on (x,y,r) x,y is coordinate, r is remain number of obstacles you can remove.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n, k) = (grid.len(), grid[0].len(), k as usize);
        let mut vis = vec![vec![vec![false; k+1]; n]; m];
        let mut q = VecDeque::new();
        q.push_back((0usize,0usize,0usize,0i32));
        vis[0][0][0] = true;
        while let Some((r,c,elim,dist)) = q.pop_front() {
            if r==m-1 && c==n-1 { return dist; }
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr=r as i32+dr; let nc=c as i32+dc;
                if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                let (nr,nc)=(nr as usize,nc as usize);
                let ne = elim + grid[nr][nc] as usize;
                if ne<=k && !vis[nr][nc][ne] { vis[nr][nc][ne]=true; q.push_back((nr,nc,ne,dist+1)); }
            }
        }
        -1
    }
}
```