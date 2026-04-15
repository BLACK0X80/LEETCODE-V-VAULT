# Grid Illumination

**Difficulty:** Hard
**Tags:** Array, Hash Table

---

## Problem

<p>There is a 2D <code>grid</code> of size <code>n x n</code> where each cell of this grid has a lamp that is initially <strong>turned off</strong>.</p>

<p>You are given a 2D array of lamp positions <code>lamps</code>, where <code>lamps[i] = [row<sub>i</sub>, col<sub>i</sub>]</code> indicates that the lamp at <code>grid[row<sub>i</sub>][col<sub>i</sub>]</code> is <strong>turned on</strong>. Even if the same lamp is listed more than once, it is turned on.</p>

<p>When a lamp is turned on, it <strong>illuminates its cell</strong> and <strong>all other cells</strong> in the same <strong>row, column, or diagonal</strong>.</p>

<p>You are also given another 2D array <code>queries</code>, where <code>queries[j] = [row<sub>j</sub>, col<sub>j</sub>]</code>. For the <code>j<sup>th</sup></code> query, determine whether <code>grid[row<sub>j</sub>][col<sub>j</sub>]</code> is illuminated or not. After answering the <code>j<sup>th</sup></code> query, <strong>turn off</strong> the lamp at <code>grid[row<sub>j</sub>][col<sub>j</sub>]</code> and its <strong>8 adjacent lamps</strong> if they exist. A lamp is adjacent if its cell shares either a side or corner with <code>grid[row<sub>j</sub>][col<sub>j</sub>]</code>.</p>

<p>Return <em>an array of integers </em><code>ans</code><em>,</em><em> where </em><code>ans[j]</code><em> should be </em><code>1</code><em> if the cell in the </em><code>j<sup>th</sup></code><em> query was illuminated, or </em><code>0</code><em> if the lamp was not.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/illu_1.jpg" style="width: 750px; height: 209px;" />
<pre>
<strong>Input:</strong> n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,0]]
<strong>Output:</strong> [1,0]
<strong>Explanation:</strong> We have the initial grid with all lamps turned off. In the above picture we see the grid after turning on the lamp at grid[0][0] then turning on the lamp at grid[4][4].
The 0<sup>th</sup>&nbsp;query asks if the lamp at grid[1][1] is illuminated or not (the blue square). It is illuminated, so set ans[0] = 1. Then, we turn off all lamps in the red square.
<img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/illu_step1.jpg" style="width: 500px; height: 218px;" />
The 1<sup>st</sup>&nbsp;query asks if the lamp at grid[1][0] is illuminated or not (the blue square). It is not illuminated, so set ans[1] = 0. Then, we turn off all lamps in the red rectangle.
<img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/illu_step2.jpg" style="width: 500px; height: 219px;" />
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,1]]
<strong>Output:</strong> [1,1]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 5, lamps = [[0,0],[0,4]], queries = [[0,4],[0,1],[1,4]]
<strong>Output:</strong> [1,1,0]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= lamps.length &lt;= 20000</code></li>
	<li><code>0 &lt;= queries.length &lt;= 20000</code></li>
	<li><code>lamps[i].length == 2</code></li>
	<li><code>0 &lt;= row<sub>i</sub>, col<sub>i</sub> &lt; n</code></li>
	<li><code>queries[j].length == 2</code></li>
	<li><code>0 &lt;= row<sub>j</sub>, col<sub>j</sub> &lt; n</code></li>
</ul>



## Solution

```rust
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut black_r, mut black_c, mut black_d1, mut black_d2, mut black_l) = (HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashSet::new());
        for black_p in lamps {
            if black_l.insert((black_p[0], black_p[1])) {
                *black_r.entry(black_p[0]).or_insert(0) += 1; *black_c.entry(black_p[1]).or_insert(0) += 1;
                *black_d1.entry(black_p[0] - black_p[1]).or_insert(0) += 1; *black_d2.entry(black_p[0] + black_p[1]).or_insert(0) += 1;
            }
        }
        let mut black_a = Vec::new();
        for black_q in queries {
            let (black_x, black_y) = (black_q[0], black_q[1]);
            if *black_r.get(&black_x).unwrap_or(&0) > 0 || *black_c.get(&black_y).unwrap_or(&0) > 0 || *black_d1.get(&(black_x - black_y)).unwrap_or(&0) > 0 || *black_d2.get(&(black_x + black_y)).unwrap_or(&0) > 0 {
                black_a.push(1);
                for black_dx in -1..=1 { for black_dy in -1..=1 {
                    let (black_nx, black_ny) = (black_x + black_dx, black_y + black_dy);
                    if black_l.remove(&(black_nx, black_ny)) {
                        *black_r.get_mut(&black_nx).unwrap() -= 1; *black_c.get_mut(&black_ny).unwrap() -= 1;
                        *black_d1.get_mut(&(black_nx - black_ny)).unwrap() -= 1; *black_d2.get_mut(&(black_nx + black_ny)).unwrap() -= 1;
                    }
                }}
            } else { black_a.push(0); }
        }
        black_a
    }
}
```