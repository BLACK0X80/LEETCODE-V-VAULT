# Making A Large Island

**Difficulty:** Hard
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>You are given an <code>n x n</code> binary matrix <code>grid</code>. You are allowed to change <strong>at most one</strong> <code>0</code> to be <code>1</code>.</p>

<p>Return <em>the size of the largest <strong>island</strong> in</em> <code>grid</code> <em>after applying this operation</em>.</p>

<p>An <strong>island</strong> is a 4-directionally connected group of <code>1</code>s.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,0],[0,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1],[1,0]]
<strong>Output:</strong> 4
<strong>Explanation: </strong>Change the 0 to 1 and make the island bigger, only one island with area = 4.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,1],[1,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Can&#39;t change any 0 to 1, only one island with area = 4.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= n &lt;= 500</code></li>
	<li><code>grid[i][j]</code> is either <code>0</code> or <code>1</code>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut sizes = vec![0i32; n*n+2];
        let mut id = 2;

        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, id: i32) -> i32 {
            let n = grid.len();
            if grid[i][j] != 1 { return 0; }
            grid[i][j] = id;
            let mut s = 1;
            if i>0{s+=dfs(grid,i-1,j,id);} if i+1<n{s+=dfs(grid,i+1,j,id);}
            if j>0{s+=dfs(grid,i,j-1,id);} if j+1<n{s+=dfs(grid,i,j+1,id);}
            s
        }

        for i in 0..n { for j in 0..n { if grid[i][j]==1 {
            sizes[id as usize] = dfs(&mut grid,i,j,id as i32);
            id += 1;
        }}}

        let mut ans = *sizes.iter().max().unwrap();
        for i in 0..n { for j in 0..n { if grid[i][j]==0 {
            let mut seen = std::collections::HashSet::new();
            let mut s = 1i32;
            for (dr,dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr=i as i32+dr; let nc=j as i32+dc;
                if nr<0||nc<0||nr>=n as i32||nc>=n as i32{continue;}
                let v = grid[nr as usize][nc as usize];
                if v>=2 && seen.insert(v) { s += sizes[v as usize]; }
            }
            ans = ans.max(s);
        }}}
        ans
    }
}
```