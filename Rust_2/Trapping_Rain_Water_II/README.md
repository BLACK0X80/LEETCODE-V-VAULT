# Trapping Rain Water II

**Difficulty:** Hard
**Tags:** Array, Breadth-First Search, Heap (Priority Queue), Matrix

---

## Problem

<p>Given an <code>m x n</code> integer matrix <code>heightMap</code> representing the height of each unit cell in a 2D elevation map, return <em>the volume of water it can trap after raining</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap1-3d.jpg" style="width: 361px; height: 321px;" />
<pre>
<strong>Input:</strong> heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> After the rain, water is trapped between the blocks.
We have two small ponds 1 and 3 units trapped.
The total volume of water trapped is 4.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/trap2-3d.jpg" style="width: 401px; height: 321px;" />
<pre>
<strong>Input:</strong> heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
<strong>Output:</strong> 10
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == heightMap.length</code></li>
	<li><code>n == heightMap[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 200</code></li>
	<li><code>0 &lt;= heightMap[i][j] &lt;= 2 * 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        if m < 3 || n < 3 { return 0; }
        let mut vis = vec![vec![false; n]; m];
        let mut pq = BinaryHeap::new();
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    vis[i][j] = true;
                    pq.push(Reverse((height_map[i][j], i, j)));
                }
            }
        }
        let mut water = 0;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while let Some(Reverse((h, r, c))) = pq.pop() {
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if !vis[nr][nc] {
                        vis[nr][nc] = true;
                        let nh = height_map[nr][nc];
                        if nh < h { water += h - nh; }
                        pq.push(Reverse((h.max(nh), nr, nc)));
                    }
                }
            }
        }
        water
    }
}
```