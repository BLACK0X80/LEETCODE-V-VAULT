# Rectangle Area II

**Difficulty:** Hard
**Tags:** Array, Segment Tree, Sweep Line, Ordered Set

---

## Problem

<p>You are given a 2D array of axis-aligned <code>rectangles</code>. Each <code>rectangle[i] = [x<sub>i1</sub>, y<sub>i1</sub>, x<sub>i2</sub>, y<sub>i2</sub>]</code> denotes the <code>i<sup>th</sup></code> rectangle where <code>(x<sub>i1</sub>, y<sub>i1</sub>)</code> are the coordinates of the <strong>bottom-left corner</strong>, and <code>(x<sub>i2</sub>, y<sub>i2</sub>)</code> are the coordinates of the <strong>top-right corner</strong>.</p>

<p>Calculate the <strong>total area</strong> covered by all <code>rectangles</code> in the plane. Any area covered by two or more rectangles should only be counted <strong>once</strong>.</p>

<p>Return <em>the <strong>total area</strong></em>. Since the answer may be too large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/06/rectangle_area_ii_pic.png" style="width: 600px; height: 450px;" />
<pre>
<strong>Input:</strong> rectangles = [[0,0,2,2],[1,0,2,3],[1,0,3,1]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> A total area of 6 is covered by all three rectangles, as illustrated in the picture.
From (1,1) to (2,2), the green and red rectangles overlap.
From (1,0) to (2,3), all three rectangles overlap.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> rectangles = [[0,0,1000000000,1000000000]]
<strong>Output:</strong> 49
<strong>Explanation:</strong> The answer is 10<sup>18</sup> modulo (10<sup>9</sup> + 7), which is 49.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= rectangles.length &lt;= 200</code></li>
	<li><code>rectanges[i].length == 4</code></li>
	<li><code>0 &lt;= x<sub>i1</sub>, y<sub>i1</sub>, x<sub>i2</sub>, y<sub>i2</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>x<sub>i1 &lt;= </sub>x<sub>i2</sub></code></li>
	<li><code>y<sub>i1 &lt;=</sub> y<sub>i2</sub></code></li>
	<li>All rectangles have non zero area.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut events = Vec::new();
        for r in &rectangles {
            let (x1, y1, x2, y2) = (r[0], r[1], r[2], r[3]);
            events.push((y1, 1, x1, x2));
            events.push((y2, -1, x1, x2));
        }
        events.sort_by_key(|e| (e.0, -e.1));
        let mut active: Vec<(i32, i32)> = Vec::new();
        let mut area = 0i64;
        let mut prev_y = events[0].0;
        for (y, typ, x1, x2) in events {
            if y != prev_y && !active.is_empty() {
                let height = (y - prev_y) as i64;
                active.sort();
                let mut width = 0i64;
                let mut cur_l = -1;
                let mut cur_r = -1;
                for &(l, r) in &active {
                    if l > cur_r {
                        if cur_r != -1 { width += (cur_r - cur_l) as i64; }
                        cur_l = l; cur_r = r;
                    } else if r > cur_r {
                        cur_r = r;
                    }
                }
                if cur_r != -1 { width += (cur_r - cur_l) as i64; }
                area = (area + width * height) % MOD;
            }
            if typ == 1 {
                active.push((x1, x2));
            } else {
                if let Some(pos) = active.iter().position(|&x| x == (x1, x2)) {
                    active.remove(pos);
                }
            }
            prev_y = y;
        }
        area as i32
    }
}
```