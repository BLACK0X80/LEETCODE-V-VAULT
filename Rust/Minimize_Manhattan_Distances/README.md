# Minimize Manhattan Distances

**Difficulty:** Hard
**Tags:** Array, Math, Geometry, Sorting, Ordered Set

---

## Problem

<p>You are given an array <code>points</code> representing integer coordinates of some points on a 2D plane, where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>.</p>

<p>The distance between two points is defined as their <span data-keyword="manhattan-distance">Manhattan distance</span>.</p>

<p>Return <em>the <strong>minimum</strong> possible value for <strong>maximum</strong> distance between any two points by removing exactly one point</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [[3,10],[5,15],[10,2],[4,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">12</span></p>

<p><strong>Explanation:</strong></p>

<p>The maximum distance after removing each point is the following:</p>

<ul>
	<li>After removing the 0<sup>th</sup> point the maximum distance is between points (5, 15) and (10, 2), which is <code>|5 - 10| + |15 - 2| = 18</code>.</li>
	<li>After removing the 1<sup>st</sup> point the maximum distance is between points (3, 10) and (10, 2), which is <code>|3 - 10| + |10 - 2| = 15</code>.</li>
	<li>After removing the 2<sup>nd</sup> point the maximum distance is between points (5, 15) and (4, 4), which is <code>|5 - 4| + |15 - 4| = 12</code>.</li>
	<li>After removing the 3<sup>rd</sup> point the maximum distance is between points (5, 15) and (10, 2), which is <code>|5 - 10| + |15 - 2| = 18</code>.</li>
</ul>

<p>12 is the minimum possible maximum distance between any two points after removing exactly one point.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [[1,1],[1,1],[1,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>Removing any of the points results in the maximum distance between any two points of 0.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= points.length &lt;= 10<sup>5</sup></code></li>
	<li><code>points[i].length == 2</code></li>
	<li><code>1 &lt;= points[i][0], points[i][1] &lt;= 10<sup>8</sup></code></li>
</ul>


## Hints

1. Notice that the Manhattan distance between two points <code>[x<sub>i</sub>, y<sub>i</sub>]</code> and <code>[x<sub>j</sub>, y<sub>j</sub>] is <code> max({x<sub>i</sub> - x<sub>j</sub> + y<sub>i</sub> - y<sub>j</sub>, x<sub>i</sub> - x<sub>j</sub> - y<sub>i</sub> + y<sub>j</sub>, - x<sub>i</sub> + x<sub>j</sub> + y<sub>i</sub> - y<sub>j</sub>, - x<sub>i</sub> + x<sub>j</sub> - y<sub>i</sub> + y<sub>j</sub>})</code></code>.
2. If you replace points as <code>[x<sub>i</sub> - y<sub>i</sub>, x<sub>i</sub> + y<sub>i</sub>]</code> then the Manhattan distance is <code>max(max(x<sub>i</sub>) - min(x<sub>i</sub>), max(y<sub>i</sub>) - min(y<sub>i</sub>))</code> over all <code>i</code>.
3. After those observations, the problem just becomes a simulation. Create multiset of points <code>[x<sub>i</sub> - y<sub>i</sub>, x<sub>i</sub> + y<sub>i</sub>]</code>, you can iterate on a point you might remove and get the maximum Manhattan distance over all other points.

## Solution

```rust
impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let u: Vec<i32> = points.iter().map(|p| p[0] + p[1]).collect();
        let v: Vec<i32> = points.iter().map(|p| p[0] - p[1]).collect();

        let max_dist = |skip: usize| -> i32 {
            let (mut umax1, mut umax2, mut umin1, mut umin2) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);
            let (mut ui1, mut ui2, mut vi1, mut vi2) = (0, 0, 0, 0);
            let (mut vmax1, mut vmax2, mut vmin1, mut vmin2) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);

            for i in 0..n {
                if i == skip { continue; }
                if u[i] > umax1 { umax2 = umax1; umax1 = u[i]; ui2 = ui1; ui1 = i; }
                else if u[i] > umax2 { umax2 = u[i]; ui2 = i; }
                if u[i] < umin1 { umin2 = umin1; umin1 = u[i]; vi2 = vi1; vi1 = i; }
                else if u[i] < umin2 { umin2 = u[i]; }
                if v[i] > vmax1 { vmax2 = vmax1; vmax1 = v[i]; }
                else if v[i] > vmax2 { vmax2 = v[i]; }
                if v[i] < vmin1 { vmin2 = vmin1; vmin1 = v[i]; }
                else if v[i] < vmin2 { vmin2 = v[i]; }
            }

            (umax1 - umin1).max(vmax1 - vmin1)
        };

        let u_sorted: Vec<(i32,usize)> = { let mut v: Vec<_> = u.iter().copied().enumerate().map(|(i,x)|(x,i)).collect(); v.sort(); v };
        let v_sorted: Vec<(i32,usize)> = { let mut v: Vec<_> = v.iter().copied().enumerate().map(|(i,x)|(x,i)).collect(); v.sort(); v };

        let candidates = vec![
            u_sorted[0].1, u_sorted[n-1].1,
            v_sorted[0].1, v_sorted[n-1].1,
        ];

        candidates.iter().map(|&skip| max_dist(skip)).min().unwrap()
    }
}
```