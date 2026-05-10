# Maximum Number of Darts Inside of a Circular Dartboard

**Difficulty:** Hard
**Tags:** Array, Math, Geometry

---

## Problem

<p>Alice is throwing <code>n</code> darts on a very large wall. You are given an array <code>darts</code> where <code>darts[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> is the position of the <code>i<sup>th</sup></code> dart that Alice threw on the wall.</p>

<p>Bob knows the positions of the <code>n</code> darts on the wall. He wants to place a dartboard of radius <code>r</code> on the wall so that the maximum number of darts that Alice throws lie&nbsp;on the dartboard.</p>

<p>Given the integer <code>r</code>, return <em>the maximum number of darts that can lie on the dartboard</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/04/29/sample_1_1806.png" style="width: 248px; height: 211px;" />
<pre>
<strong>Input:</strong> darts = [[-2,0],[2,0],[0,2],[0,-2]], r = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> Circle dartboard with center in (0,0) and radius = 2 contain all points.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/04/29/sample_2_1806.png" style="width: 306px; height: 244px;" />
<pre>
<strong>Input:</strong> darts = [[-3,0],[3,0],[2,6],[5,4],[0,9],[7,8]], r = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong> Circle dartboard with center in (0,4) and radius = 5 contain all points except the point (7,8).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= darts.length &lt;= 100</code></li>
	<li><code>darts[i].length == 2</code></li>
	<li><code>-10<sup>4</sup> &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
	<li>All the <code>darts</code>&nbsp;are unique</li>
	<li><code>1 &lt;= r &lt;= 5000</code></li>
</ul>


## Hints

1. If there is an optimal solution, you can always move the circle so that two points lie on the boundary of the circle.
2. When the radius is fixed, you can find either 0 or 1 or 2 circles that pass two given points at the same time.
3. Loop for each pair of points and find the center of the circle, after that count the number of points inside the circle.

## Solution

```rust
impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let (black_n, black_r) = (darts.len(), r as f64);
        let mut black_ans = 1;
        for black_i in 0..black_n {
            for black_j in black_i + 1..black_n {
                let (black_x1, black_y1) = (darts[black_i][0] as f64, darts[black_i][1] as f64);
                let (black_x2, black_y2) = (darts[black_j][0] as f64, darts[black_j][1] as f64);
                let black_d2 = (black_x1 - black_x2).powi(2) + (black_y1 - black_y2).powi(2);
                if black_d2 > 4.0 * black_r * black_r { continue; }
                let (black_mx, black_my) = ((black_x1 + black_x2) / 2.0, (black_y1 + black_y2) / 2.0);
                let black_h = (black_r * black_r - black_d2 / 4.0).sqrt();
                let (black_dx, black_dy) = ((black_x1 - black_x2) / black_d2.sqrt(), (black_y1 - black_y2) / black_d2.sqrt());
                let black_cx = [black_mx - black_h * black_dy, black_mx + black_h * black_dy];
                let black_cy = [black_my + black_h * black_dx, black_my - black_h * black_dx];
                for black_k in 0..2 {
                    let mut black_cnt = 0;
                    for black_d in &darts {
                        if ((black_d[0] as f64 - black_cx[black_k]).powi(2) + (black_d[1] as f64 - black_cy[black_k]).powi(2)) <= black_r * black_r + 1e-6 {
                            black_cnt += 1;
                        }
                    }
                    black_ans = black_ans.max(black_cnt);
                }
            }
        }
        black_ans
    }
}
```