# Unique Paths III

**Difficulty:** Hard
**Tags:** Array, Backtracking, Bit Manipulation, Matrix

---

## Problem

<p>You are given an <code>m x n</code> integer array <code>grid</code> where <code>grid[i][j]</code> could be:</p>

<ul>
	<li><code>1</code> representing the starting square. There is exactly one starting square.</li>
	<li><code>2</code> representing the ending square. There is exactly one ending square.</li>
	<li><code>0</code> representing empty squares we can walk over.</li>
	<li><code>-1</code> representing obstacles that we cannot walk over.</li>
</ul>

<p>Return <em>the number of 4-directional walks from the starting square to the ending square, that walk over every non-obstacle square exactly once</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/02/lc-unique1.jpg" style="width: 324px; height: 245px;" />
<pre>
<strong>Input:</strong> grid = [[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We have the following two paths: 
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/02/lc-unique2.jpg" style="width: 324px; height: 245px;" />
<pre>
<strong>Input:</strong> grid = [[1,0,0,0],[0,0,0,0],[0,0,0,2]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> We have the following four paths: 
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/02/lc-unique3-.jpg" style="width: 164px; height: 165px;" />
<pre>
<strong>Input:</strong> grid = [[0,1],[2,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no path that walks over every empty square exactly once.
Note that the starting and ending square can be anywhere in the grid.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 20</code></li>
	<li><code>1 &lt;= m * n &lt;= 20</code></li>
	<li><code>-1 &lt;= grid[i][j] &lt;= 2</code></li>
	<li>There is exactly one starting cell and one ending cell.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut sr, mut sc, mut empty) = (0, 0, 1);
        for i in 0..m { for j in 0..n { match grid[i][j] { 1=>{sr=i;sc=j;} 0=>{empty+=1;} _=>{} } } }

        fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, left: i32) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            if grid[r][c] == 2 { return if left == 0 { 1 } else { 0 }; }
            let tmp = grid[r][c]; grid[r][c] = -1;
            let mut res = 0;
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr=r as i32+dr; let nc=c as i32+dc;
                if nr<0||nc<0||nr>=m as i32||nc>=n as i32 { continue; }
                let (nr,nc)=(nr as usize,nc as usize);
                if grid[nr][nc] != -1 { res += dfs(grid, nr, nc, left-1); }
            }
            grid[r][c] = tmp;
            res
        }

        dfs(&mut grid, sr, sc, empty)
    }
}
```