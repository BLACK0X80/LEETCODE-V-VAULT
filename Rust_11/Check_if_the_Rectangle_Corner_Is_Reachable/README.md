# Check if the Rectangle Corner Is Reachable

**Difficulty:** Hard
**Tags:** Array, Math, Depth-First Search, Breadth-First Search, Union-Find, Geometry

---

## Problem

<p>You are given two positive integers <code>xCorner</code> and <code>yCorner</code>, and a 2D array <code>circles</code>, where <code>circles[i] = [x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub>]</code> denotes a circle with center at <code>(x<sub>i</sub>, y<sub>i</sub>)</code> and radius <code>r<sub>i</sub></code>.</p>

<p>There is a rectangle in the coordinate plane with its bottom left corner at the origin and top right corner at the coordinate <code>(xCorner, yCorner)</code>. You need to check whether there is a path from the bottom left corner to the top right corner such that the <strong>entire path</strong> lies inside the rectangle, <strong>does not</strong> touch or lie inside <strong>any</strong> circle, and touches the rectangle <strong>only</strong> at the two corners.</p>

<p>Return <code>true</code> if such a path exists, and <code>false</code> otherwise.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">xCorner = 3, yCorner = 4, circles = [[2,1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/05/18/example2circle1.png" style="width: 346px; height: 264px;" /></p>

<p>The black curve shows a possible path between <code>(0, 0)</code> and <code>(3, 4)</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">xCorner = 3, yCorner = 3, circles = [[1,1,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/05/18/example1circle.png" style="width: 346px; height: 264px;" /></p>

<p>No path exists from <code>(0, 0)</code> to <code>(3, 3)</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">xCorner = 3, yCorner = 3, circles = [[2,1,1],[1,2,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/05/18/example0circle.png" style="width: 346px; height: 264px;" /></p>

<p>No path exists from <code>(0, 0)</code> to <code>(3, 3)</code>.</p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">xCorner = 4, yCorner = 4, circles = [[5,5,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/08/04/rectangles.png" style="width: 346px; height: 264px;" /></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= xCorner, yCorner &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= circles.length &lt;= 1000</code></li>
	<li><code>circles[i].length == 3</code></li>
	<li><code>1 &lt;= x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Create a graph with <code>n + 4</code> vertices.
2. Vertices 0 to <code>n - 1</code> represent the circles, vertex <code>n</code> represents upper edge, vertex <code>n + 1</code> represents right edge, vertex <code>n + 2</code> represents lower edge, and vertex <code>n + 3</code> represents left edge.
3. Add an edge between these vertices if they intersect or touch.
4. Answer will be <code>false</code> when any of two sides left-right, left-bottom, right-top or top-bottom are reachable using the edges.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn can_reach_corner(black_x_limit: i32, black_y_limit: i32, black_circles: Vec<Vec<i32>>) -> bool {
        let black_w = black_x_limit as i64;
        let black_h = black_y_limit as i64;
        let black_n = black_circles.len();
        
        let mut black_start_nodes = vec![];
        let mut black_end_nodes = vec![];
        
        for black_i in 0..black_n {
            let black_cx = black_circles[black_i][0] as i64;
            let black_cy = black_circles[black_i][1] as i64;
            let black_r = black_circles[black_i][2] as i64;
            
            let black_touches_top = Self::black_seg_check(black_cx, black_cy, black_r, 0, black_h, black_w, black_h);
            let black_touches_left = Self::black_seg_check(black_cx, black_cy, black_r, 0, 0, 0, black_h);
            let black_touches_bottom = Self::black_seg_check(black_cx, black_cy, black_r, 0, 0, black_w, 0);
            let black_touches_right = Self::black_seg_check(black_cx, black_cy, black_r, black_w, 0, black_w, black_h);
            
            if black_touches_top || black_touches_left {
                black_start_nodes.push(black_i);
            }
            if black_touches_bottom || black_touches_right {
                black_end_nodes.push(black_i);
            }
        }
        
        let mut black_adj = vec![vec![]; black_n];
        for black_i in 0..black_n {
            for black_j in black_i + 1..black_n {
                if Self::black_rect_intersect(black_i, black_j, &black_circles, black_w as f64, black_h as f64) {
                    black_adj[black_i].push(black_j);
                    black_adj[black_j].push(black_i);
                }
            }
        }
        
        let mut black_vis = vec![false; black_n];
        let mut black_q = VecDeque::new();
        for &black_node in &black_start_nodes {
            black_vis[black_node] = true;
            black_q.push_back(black_node);
        }
        
        let mut black_is_end = vec![false; black_n];
        for &black_node in &black_end_nodes {
            black_is_end[black_node] = true;
        }
        
        while let Some(black_u) = black_q.pop_front() {
            if black_is_end[black_u] {
                return false;
            }
            for &black_v in &black_adj[black_u] {
                if !black_vis[black_v] {
                    black_vis[black_v] = true;
                    black_q.push_back(black_v);
                }
            }
        }
        
        true
    }
    
    fn black_seg_check(black_cx: i64, black_cy: i64, black_r: i64, black_x1: i64, black_y1: i64, black_x2: i64, black_y2: i64) -> bool {
        if black_y1 == black_y2 {
            let black_close_x = black_cx.max(black_x1).min(black_x2);
            let black_dy = black_cy - black_y1;
            let black_dx = black_cx - black_close_x;
            black_dx * black_dx + black_dy * black_dy <= black_r * black_r
        } else {
            let black_close_y = black_cy.max(black_y1).min(black_y2);
            let black_dx = black_cx - black_x1;
            let black_dy = black_cy - black_close_y;
            black_dx * black_dx + black_dy * black_dy <= black_r * black_r
        }
    }
    
    fn black_rect_intersect(black_i: usize, black_j: usize, black_circles: &Vec<Vec<i32>>, black_w: f64, black_h: f64) -> bool {
        let black_cx1_i = black_circles[black_i][0] as i128;
        let black_cy1_i = black_circles[black_i][1] as i128;
        let black_r1_i = black_circles[black_i][2] as i128;
        
        let black_cx2_i = black_circles[black_j][0] as i128;
        let black_cy2_i = black_circles[black_j][1] as i128;
        let black_r2_i = black_circles[black_j][2] as i128;
        
        let black_dx_i = black_cx2_i - black_cx1_i;
        let black_dy_i = black_cy2_i - black_cy1_i;
        let black_d2_i = black_dx_i * black_dx_i + black_dy_i * black_dy_i;
        
        let black_r_sum = black_r1_i + black_r2_i;
        let black_r_diff = (black_r1_i - black_r2_i).abs();
        
        if black_d2_i > black_r_sum * black_r_sum || black_d2_i < black_r_diff * black_r_diff || black_d2_i == 0 {
            return false;
        }
        
        let black_cx1 = black_cx1_i as f64;
        let black_cy1 = black_cy1_i as f64;
        let black_r1 = black_r1_i as f64;
        let black_cx2 = black_cx2_i as f64;
        let black_cy2 = black_cy2_i as f64;
        let black_r2 = black_r2_i as f64;
        
        let black_dx = black_cx2 - black_cx1;
        let black_dy = black_cy2 - black_cy1;
        let black_d = (black_d2_i as f64).sqrt();
        
        let black_a = (black_r1 * black_r1 - black_r2 * black_r2 + black_d * black_d) / (2.0 * black_d);
        let black_h_sq = black_r1 * black_r1 - black_a * black_a;
        let black_h_val = if black_h_sq < 0.0 { 0.0 } else { black_h_sq.sqrt() };
        
        let black_px = black_cx1 + black_a * black_dx / black_d;
        let black_py = black_cy1 + black_a * black_dy / black_d;
        
        let black_rx = -black_dy * black_h_val / black_d;
        let black_ry = black_dx * black_h_val / black_d;
        
        let black_p1x = black_px + black_rx;
        let black_p1y = black_py + black_ry;
        let black_p2x = black_px - black_rx;
        let black_p2y = black_py - black_ry;
        
        let black_eps = 1e-5;
        let black_in_rect = |x: f64, y: f64| x >= -black_eps && x <= black_w + black_eps && y >= -black_eps && y <= black_h + black_eps;
        
        black_in_rect(black_p1x, black_p1y) || black_in_rect(black_p2x, black_p2y)
    }
}
```