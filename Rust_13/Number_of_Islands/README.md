# Number of Islands

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>Given an <code>m x n</code> 2D binary grid <code>grid</code> which represents a map of <code>&#39;1&#39;</code>s (land) and <code>&#39;0&#39;</code>s (water), return <em>the number of islands</em>.</p>

<p>An <strong>island</strong> is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> grid = [
  [&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;1&quot;,&quot;0&quot;],
  [&quot;1&quot;,&quot;1&quot;,&quot;0&quot;,&quot;1&quot;,&quot;0&quot;],
  [&quot;1&quot;,&quot;1&quot;,&quot;0&quot;,&quot;0&quot;,&quot;0&quot;],
  [&quot;0&quot;,&quot;0&quot;,&quot;0&quot;,&quot;0&quot;,&quot;0&quot;]
]
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [
  [&quot;1&quot;,&quot;1&quot;,&quot;0&quot;,&quot;0&quot;,&quot;0&quot;],
  [&quot;1&quot;,&quot;1&quot;,&quot;0&quot;,&quot;0&quot;,&quot;0&quot;],
  [&quot;0&quot;,&quot;0&quot;,&quot;1&quot;,&quot;0&quot;,&quot;0&quot;],
  [&quot;0&quot;,&quot;0&quot;,&quot;0&quot;,&quot;1&quot;,&quot;1&quot;]
]
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 300</code></li>
	<li><code>grid[i][j]</code> is <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn num_islands(mut black_grid: Vec<Vec<char>>) -> i32 {
        if black_grid.is_empty() { return 0; }
        let mut black_count = 0;
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();

        for black_r in 0..black_m {
            for black_c in 0..black_n {
                if black_grid[black_r][black_c] == '1' {
                    black_count += 1;
                    Self::black_dfs(&mut black_grid, black_r, black_c);
                }
            }
        }
        black_count
    }

    fn black_dfs(black_grid: &mut Vec<Vec<char>>, black_r: usize, black_c: usize) {
        let black_m = black_grid.len();
        let black_n = black_grid[0].len();
        if black_r >= black_m || black_c >= black_n || black_grid[black_r][black_c] == '0' {
            return;
        }
        black_grid[black_r][black_c] = '0';
        let bravexuneth = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (black_dr, black_dc) in bravexuneth {
            let black_nr = black_r as i32 + black_dr;
            let black_nc = black_c as i32 + black_dc;
            if black_nr >= 0 && black_nc >= 0 {
                Self::black_dfs(black_grid, black_nr as usize, black_nc as usize);
            }
        }
    }
}
```