# Max Points on a Line

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Geometry

---

## Problem

<p>Given an array of <code>points</code> where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents a point on the <strong>X-Y</strong> plane, return <em>the maximum number of points that lie on the same straight line</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg" style="width: 300px; height: 294px;" />
<pre>
<strong>Input:</strong> points = [[1,1],[2,2],[3,3]]
<strong>Output:</strong> 3
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg" style="width: 300px; height: 294px;" />
<pre>
<strong>Input:</strong> points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
<strong>Output:</strong> 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= points.length &lt;= 300</code></li>
	<li><code>points[i].length == 2</code></li>
	<li><code>-10<sup>4</sup> &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
	<li>All the <code>points</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 { return n as i32; }
        let mut black = 2;
        fn gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { gcd(b, a%b) } }
        for b in 0..n {
            let mut map: HashMap<(i32,i32), i32> = HashMap::new();
            for bl in b+1..n {
                let dy = points[bl][1] - points[b][1];
                let dx = points[bl][0] - points[b][0];
                let g = gcd(dy.abs(), dx.abs());
                let key = if dx == 0 { (1,0) } else if dy == 0 { (0,1) } else {
                    let (dy,dx) = (dy/g, dx/g);
                    if dx < 0 { (-dy,-dx) } else { (dy,dx) }
                };
                let cnt = map.entry(key).or_insert(0);
                *cnt += 1;
                black = black.max(*cnt + 1);
            }
        }
        black
    }
}
```