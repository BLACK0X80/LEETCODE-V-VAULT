# Perfect Rectangle

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Geometry, Sweep Line

---

## Problem

<p>Given an array <code>rectangles</code> where <code>rectangles[i] = [x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub>]</code> represents an axis-aligned rectangle. The bottom-left point of the rectangle is <code>(x<sub>i</sub>, y<sub>i</sub>)</code> and the top-right point of it is <code>(a<sub>i</sub>, b<sub>i</sub>)</code>.</p>

<p>Return <code>true</code> <em>if all the rectangles together form an exact cover of a rectangular region</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perectrec1-plane.jpg" style="width: 300px; height: 294px;" />
<pre>
<strong>Input:</strong> rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
<strong>Output:</strong> true
<strong>Explanation:</strong> All 5 rectangles together form an exact cover of a rectangular region.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfectrec2-plane.jpg" style="width: 300px; height: 294px;" />
<pre>
<strong>Input:</strong> rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
<strong>Output:</strong> false
<strong>Explanation:</strong> Because there is a gap between the two rectangular regions.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/03/27/perfecrrec4-plane.jpg" style="width: 300px; height: 294px;" />
<pre>
<strong>Input:</strong> rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
<strong>Output:</strong> false
<strong>Explanation:</strong> Because two of the rectangles overlap with each other.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= rectangles.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>rectangles[i].length == 4</code></li>
	<li><code>-10<sup>5</sup> &lt;= x<sub>i</sub> &lt; a<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>5</sup> &lt;= y<sub>i</sub> &lt; b<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut area = 0i64;
        let mut corners: HashSet<(i32,i32)> = HashSet::new();
        let (mut x1, mut y1, mut x2, mut y2) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for r in &rectangles {
            area += (r[2]-r[0]) as i64 * (r[3]-r[1]) as i64;
            x1=x1.min(r[0]); y1=y1.min(r[1]); x2=x2.max(r[2]); y2=y2.max(r[3]);
            for p in [(r[0],r[1]),(r[0],r[3]),(r[2],r[1]),(r[2],r[3])] {
                if !corners.insert(p) { corners.remove(&p); }
            }
        }
        area == (x2-x1) as i64 * (y2-y1) as i64
            && corners.len() == 4
            && corners.contains(&(x1,y1)) && corners.contains(&(x1,y2))
            && corners.contains(&(x2,y1)) && corners.contains(&(x2,y2))
    }
}
```