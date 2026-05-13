# Count Number of Trapezoids II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Geometry

---

## Problem

<p data-end="189" data-start="146">You are given a 2D integer array <code>points</code> where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents the coordinates of the <code>i<sup>th</sup></code> point on the Cartesian plane.</p>

<p data-end="189" data-start="146">Return <em data-end="330" data-start="297">the number of unique </em><em>trapezoids</em> that can be formed by choosing any four distinct points from <code>points</code>.</p>

<p data-end="579" data-start="405">A<b> </b><strong>trapezoid</strong> is a convex quadrilateral with <strong data-end="496" data-start="475">at least one pair</strong> of parallel sides. Two lines are parallel if and only if they have the same slope.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [[-3,2],[3,0],[2,3],[3,2],[2,-3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/04/29/desmos-graph-4.png" style="width: 250px; height: 250px;" /> <img alt="" src="https://assets.leetcode.com/uploads/2025/04/29/desmos-graph-3.png" style="width: 250px; height: 250px;" /></p>

<p>There are two distinct ways to pick four points that form a trapezoid:</p>

<ul>
	<li>The points <code>[-3,2], [2,3], [3,2], [2,-3]</code> form one trapezoid.</li>
	<li>The points <code>[2,3], [3,2], [3,0], [2,-3]</code> form another trapezoid.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [[0,0],[1,0],[0,1],[2,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/04/29/desmos-graph-5.png" style="width: 250px; height: 250px;" /></p>

<p>There is only one trapezoid which can be formed.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>4 &lt;= points.length &lt;= 500</code></li>
	<li><code>&ndash;1000 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 1000</code></li>
	<li>All points are pairwise distinct.</li>
</ul>


## Hints

1. Hash every point-pair by its reduced slope <code>(dy,dx)</code> (normalize with GCD and fix signs).
2. In each slope-bucket of size <code>k</code>, there are <code>C(k,2)</code> ways to pick two segments as the trapezoid's parallel bases.
3. Skip any base-pair that shares an endpoint since it would not form a quadrilateral.
4. Subtract one count for each parallelogram. Each parallelogram was counted once for each of its two parallel-side pairs, so after subtracting once, every quadrilateral with at least one pair of parallel sides, including parallelograms, contributes exactly one to the final total.
5. Final answer = total valid base-pairs minus parallelogram overcounts.

## Solution

```rust
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_trapezoids(black_pts: Vec<Vec<i32>>) -> i64 {
        let black_n = black_pts.len();
        let mut black_mid_map: HashMap<i64, (i64, HashMap<i64, i32>)> = HashMap::new();
        let mut black_slope_map: HashMap<i64, HashMap<i64, HashSet<usize>>> = HashMap::new();

        fn black_gcd(a: i32, b: i32) -> i32 { if b == 0 { a.abs() } else { black_gcd(b, a % b) } }

        for i in 0..black_n {
            for j in i + 1..black_n {
                let (x1, y1) = (black_pts[i][0], black_pts[i][1]);
                let (x2, y2) = (black_pts[j][0], black_pts[j][1]);

                let (mut dx, mut dy) = (x2 - x1, y2 - y1);
                let g = black_gcd(dx, dy);
                dx /= g; dy /= g;
                if dx < 0 || (dx == 0 && dy < 0) { dx = -dx; dy = -dy; }

                let black_s_key = ((dy as i64 + 2000) << 32) | (dx as i64 + 2000);
                let black_l_key = dy as i64 * x1 as i64 - dx as i64 * y1 as i64;
                black_slope_map.entry(black_s_key).or_default().entry(black_l_key).or_default().extend([i, j]);

                let black_m_key = (((x1 + x2) as i64 + 4000) << 32) | ((y1 + y2) as i64 + 4000);
                let black_mid_info = black_mid_map.entry(black_m_key).or_insert((0, HashMap::new()));
                black_mid_info.0 += 1;
                *black_mid_info.1.entry(black_s_key).or_default() += 1;
            }
        }

        let mut black_para = 0i64;
        for (_, (black_tot, black_dirs)) in black_mid_map {
            if black_tot < 2 { continue; }
            let mut black_bad = 0i64;
            for &c in black_dirs.values() { black_bad += c as i64 * (c as i64 - 1) / 2; }
            black_para += black_tot * (black_tot - 1) / 2 - black_bad;
        }

        let mut black_total_para = 0i64;
        for black_line_map in black_slope_map.values() {
            let black_a: Vec<i64> = black_line_map.values()
                .filter(|v| v.len() >= 2)
                .map(|v| { let c = v.len() as i64; c * (c - 1) / 2 })
                .collect();
            if black_a.len() < 2 { continue; }
            let (mut s, mut s2) = (0i64, 0i64);
            for x in black_a { s += x; s2 += x * x; }
            black_total_para += (s * s - s2) / 2;
        }

        black_total_para - black_para
    }
}
```