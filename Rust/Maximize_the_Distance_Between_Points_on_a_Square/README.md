# Maximize the Distance Between Points on a Square

**Difficulty:** Hard
**Tags:** Array, Math, Binary Search, Geometry, Sorting

---

## Problem

<p>You are given an integer <code><font face="monospace">side</font></code>, representing the edge length of a square with corners at <code>(0, 0)</code>, <code>(0, side)</code>, <code>(side, 0)</code>, and <code>(side, side)</code> on a Cartesian plane.</p>

<p>You are also given a <strong>positive</strong> integer <code>k</code> and a 2D integer array <code>points</code>, where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents the coordinate of a point lying on the <strong>boundary</strong> of the square.</p>

<p>You need to select <code>k</code> elements among <code>points</code> such that the <strong>minimum</strong> Manhattan distance between any two points is <strong>maximized</strong>.</p>

<p>Return the <strong>maximum</strong> possible <strong>minimum</strong> Manhattan distance between the selected <code>k</code> points.</p>

<p>The Manhattan Distance between two cells <code>(x<sub>i</sub>, y<sub>i</sub>)</code> and <code>(x<sub>j</sub>, y<sub>j</sub>)</code> is <code>|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">side = 2, points = [[0,2],[2,0],[2,2],[0,0]], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/01/28/4080_example0_revised.png" style="width: 200px; height: 200px;" /></p>

<p>Select all four points.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">side = 2, points = [[0,0],[1,2],[2,0],[2,2],[2,1]], k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/01/28/4080_example1_revised.png" style="width: 211px; height: 200px;" /></p>

<p>Select the points <code>(0, 0)</code>, <code>(2, 0)</code>, <code>(2, 2)</code>, and <code>(2, 1)</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">side = 2, points = [[0,0],[0,1],[0,2],[1,2],[2,0],[2,2],[2,1]], k = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/01/28/4080_example2_revised.png" style="width: 200px; height: 200px;" /></p>

<p>Select the points <code>(0, 0)</code>, <code>(0, 1)</code>, <code>(0, 2)</code>, <code>(1, 2)</code>, and <code>(2, 2)</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= side &lt;= 10<sup>9</sup></code></li>
	<li><code>4 &lt;= points.length &lt;= min(4 * side, 15 * 10<sup>3</sup>)</code></li>
	<li><code>points[i] == [xi, yi]</code></li>
	<li>The input is generated such that:
	<ul>
		<li><code>points[i]</code> lies on the boundary of the square.</li>
		<li>All <code>points[i]</code> are <strong>unique</strong>.</li>
	</ul>
	</li>
	<li><code>4 &lt;= k &lt;= min(25, points.length)</code></li>
</ul>


## Hints

1. Can we use binary search for this problem?
2. Think of the coordinates on a straight line in clockwise order.
3. Binary search on the minimum Manhattan distance <code>x</code>.
4. During the binary search, for each coordinate, find the immediate next coordinate with distance >= <code>x</code>.
5. Greedily select up to <code>k</code> coordinates.

## Solution

```rust
impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let s = side as i64;
        let k = k as usize;
        let mut black_pts: Vec<i64> = points.iter().map(|p| {
            let (x, y) = (p[0] as i64, p[1] as i64);
            if y == 0 { x }
            else if x == s { s + y }
            else if y == s { 2*s + (s-x) }
            else { 3*s + (s-y) }
        }).collect();
        black_pts.sort_unstable();
        let n = black_pts.len();
        let perim = 4*s;

        let black_check = |mid: i64| -> bool {
            for start in 0..n {
                let mut cur = black_pts[start];
                let mut cnt = 1;
                let mut ok = true;
                while cnt < k {
                    let need = cur + mid;
                    let idx = black_pts.partition_point(|&x| x < need);
                    if idx < n {
                        cur = black_pts[idx];
                        cnt += 1;
                    } else {
                        ok = false; break;
                    }
                }
                if ok {
                    let wrap = black_pts[start] + perim - cur;
                    if wrap >= mid { return true; }
                }
            }
            false
        };

        let (mut lo, mut hi) = (0i64, perim / k as i64);
        while lo < hi {
            let mid = (lo+hi+1)/2;
            if black_check(mid) { lo = mid; } else { hi = mid-1; }
        }
        lo as i32
    }
}
```