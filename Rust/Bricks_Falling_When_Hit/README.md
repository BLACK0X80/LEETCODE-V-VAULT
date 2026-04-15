# Bricks Falling When Hit

**Difficulty:** Hard
**Tags:** Array, Union-Find, Matrix

---

## Problem

<p>You are given an <code>m x n</code> binary <code>grid</code>, where each <code>1</code> represents a brick and <code>0</code> represents an empty space. A brick is <strong>stable</strong> if:</p>

<ul>
	<li>It is directly connected to the top of the grid, or</li>
	<li>At least one other brick in its four adjacent cells is <strong>stable</strong>.</li>
</ul>

<p>You are also given an array <code>hits</code>, which is a sequence of erasures we want to apply. Each time we want to erase the brick at the location <code>hits[i] = (row<sub>i</sub>, col<sub>i</sub>)</code>. The brick on that location&nbsp;(if it exists) will disappear. Some other bricks may no longer be stable because of that erasure and will <strong>fall</strong>. Once a brick falls, it is <strong>immediately</strong> erased from the <code>grid</code> (i.e., it does not land on other stable bricks).</p>

<p>Return <em>an array </em><code>result</code><em>, where each </em><code>result[i]</code><em> is the number of bricks that will <strong>fall</strong> after the </em><code>i<sup>th</sup></code><em> erasure is applied.</em></p>

<p><strong>Note</strong> that an erasure may refer to a location with no brick, and if it does, no bricks drop.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,0,0,0],[1,1,1,0]], hits = [[1,0]]
<strong>Output:</strong> [2]
<strong>Explanation: </strong>Starting with the grid:
[[1,0,0,0],
 [<u>1</u>,1,1,0]]
We erase the underlined brick at (1,0), resulting in the grid:
[[1,0,0,0],
 [0,<u>1</u>,<u>1</u>,0]]
The two underlined bricks are no longer stable as they are no longer connected to the top nor adjacent to another stable brick, so they will fall. The resulting grid is:
[[1,0,0,0],
 [0,0,0,0]]
Hence the result is [2].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[1,0,0,0],[1,1,0,0]], hits = [[1,1],[1,0]]
<strong>Output:</strong> [0,0]
<strong>Explanation: </strong>Starting with the grid:
[[1,0,0,0],
 [1,<u>1</u>,0,0]]
We erase the underlined brick at (1,1), resulting in the grid:
[[1,0,0,0],
 [1,0,0,0]]
All remaining bricks are still stable, so no bricks fall. The grid remains the same:
[[1,0,0,0],
 [<u>1</u>,0,0,0]]
Next, we erase the underlined brick at (1,0), resulting in the grid:
[[1,0,0,0],
 [0,0,0,0]]
Once again, all remaining bricks are still stable, so no bricks fall.
Hence the result is [0,0].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 200</code></li>
	<li><code>grid[i][j]</code> is <code>0</code> or <code>1</code>.</li>
	<li><code>1 &lt;= hits.length &lt;= 4 * 10<sup>4</sup></code></li>
	<li><code>hits[i].length == 2</code></li>
	<li><code>0 &lt;= x<sub>i&nbsp;</sub>&lt;= m - 1</code></li>
	<li><code>0 &lt;=&nbsp;y<sub>i</sub> &lt;= n - 1</code></li>
	<li>All <code>(x<sub>i</sub>, y<sub>i</sub>)</code> are unique.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let top = m * n;
        let mut par: Vec<usize> = (0..=top).collect();
        let mut rnk = vec![0u8; top + 1];
        let mut sz = vec![1usize; top + 1];

        fn find(par: &mut Vec<usize>, x: usize) -> usize {
            if par[x] != x { par[x] = find(par, par[x]); }
            par[x]
        }
        fn union(par: &mut Vec<usize>, rnk: &mut Vec<u8>, sz: &mut Vec<usize>, x: usize, y: usize) {
            let (mut rx, mut ry) = (find(par, x), find(par, y));
            if rx == ry { return; }
            if rnk[rx] < rnk[ry] { std::mem::swap(&mut rx, &mut ry); }
            if rnk[rx] == rnk[ry] { rnk[rx] += 1; }
            par[ry] = rx;
            sz[rx] += sz[ry];
        }

        let idx = |r: usize, c: usize| r * n + c;
        let mut a = grid.clone();
        for h in &hits { a[h[0] as usize][h[1] as usize] = 0; }

        for r in 0..m {
            for c in 0..n {
                if a[r][c] == 1 {
                    let i = idx(r, c);
                    if r == 0 { union(&mut par, &mut rnk, &mut sz, i, top); }
                    if r > 0 && a[r-1][c] == 1 { union(&mut par, &mut rnk, &mut sz, i, idx(r-1,c)); }
                    if c > 0 && a[r][c-1] == 1 { union(&mut par, &mut rnk, &mut sz, i, idx(r,c-1)); }
                }
            }
        }

        let mut ans = vec![0i32; hits.len()];
        for i in (0..hits.len()).rev() {
            let (r, c) = (hits[i][0] as usize, hits[i][1] as usize);
            let pre = sz[find(&mut par, top)];
            if grid[r][c] == 0 { continue; }
            let ci = idx(r, c);
            for (dr, dc) in [(0i32,1i32),(0,-1),(1,0),(-1,0)] {
                let nr = r as i32 + dr; let nc = c as i32 + dc;
                if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 { continue; }
                let (nr, nc) = (nr as usize, nc as usize);
                if a[nr][nc] == 1 { union(&mut par, &mut rnk, &mut sz, ci, idx(nr,nc)); }
            }
            if r == 0 { union(&mut par, &mut rnk, &mut sz, ci, top); }
            a[r][c] = 1;
            ans[i] = (sz[find(&mut par, top)] as i32 - pre as i32 - 1).max(0);
        }
        ans
    }
}
```