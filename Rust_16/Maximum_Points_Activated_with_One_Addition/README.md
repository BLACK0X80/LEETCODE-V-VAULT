# Maximum Points Activated with One Addition

**Difficulty:** Hard
**Tags:** Array, Hash Table, Union-Find

---

## Problem

<p>You are given a 2D integer array <code>points</code>, where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> represents the coordinates of the <code>i<sup>th</sup></code> point. All coordinates in <code>points</code> are <strong>distinct</strong>.</p>

<p>If a point is <strong>activated</strong>, then all points that have the <strong>same</strong> x-coordinate <strong>or</strong> y-coordinate become <strong>activated</strong> as well.</p>

<p>Activation continues until no additional points can be activated.</p>

<p>You may add <strong>one additional</strong> point at any integer coordinate <code>(x, y)</code> not already present in <code>points</code>. Activation begins by <strong>activating</strong> this <strong>newly added point</strong>.</p>

<p>Return an integer denoting the <strong>maximum</strong> number of points that can be activated, including the newly added point.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [[1,1],[1,2],[2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Adding and activating a point such as <code>(1, 3)</code> causes activations:</p>

<ul>
	<li><code>(1, 3)</code> shares <code>x = 1</code> with <code>(1, 1)</code> and <code>(1, 2)</code> -&gt; <code>(1, 1)</code> and <code>(1, 2)</code> become activated.</li>
	<li><code>(1, 2)</code> shares <code>y = 2</code> with <code>(2, 2)</code> -&gt; <code>(2, 2)</code> becomes activated.</li>
</ul>

<p>Thus, the activated points are <code>(1, 3)</code>, <code>(1, 1)</code>, <code>(1, 2)</code>, <code>(2, 2)</code>, so 4 points in total. We can show this is the maximum activated.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [[2,2],[1,1],[3,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>Adding and activating a point such as <code>(1, 2)</code> causes activations:</p>

<ul>
	<li><code>(1, 2)</code> shares <code>x = 1</code> with <code>(1, 1)</code> -&gt; <code>(1, 1)</code> becomes activated.</li>
	<li><code>(1, 2)</code> shares <code>y = 2</code> with <code>(2, 2)</code> -&gt; <code>(2, 2)</code> becomes activated.</li>
</ul>

<p>Thus, the activated points are <code>(1, 2)</code>, <code>(1, 1)</code>, <code>(2, 2)</code>, so 3 points in total. We can show this is the maximum activated.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">points = [[2,3],[2,2],[1,1],[4,5]]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>Adding and activating a point such as <code>(2, 1)</code> causes activations:</p>

<ul>
	<li><code>(2, 1)</code> shares <code>x = 2</code> with <code>(2, 3)</code> and <code>(2, 2)</code> -&gt; <code>(2, 3)</code> and <code>(2, 2)</code> become activated.</li>
	<li><code>(2, 1)</code> shares <code>y = 1</code> with <code>(1, 1)</code> -&gt; <code>(1, 1)</code> becomes activated.</li>
</ul>

<p>Thus, the activated points are <code>(2, 1)</code>, <code>(2, 3)</code>, <code>(2, 2)</code>, <code>(1, 1)</code>, so 4 points in total.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= points.length &lt;= 10<sup>5</sup></code></li>
	<li><code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code></li>
	<li><code>-10<sup>9</sup> &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>points</code> contains all <strong>distinct</strong> coordinates.</li>
</ul>


## Hints

1. Use disjoint-set union (DSU).
2. Build components by unioning points that share the same <code>x</code> or the same <code>y</code>.
3. Each <code>x</code> maps to one component and each <code>y</code> maps to one component. Adding <code>(x0, y0)</code> connects at most two distinct components; activated = <code>size(A) + size(B) + 1</code> (if same component then <code>size(A) + 1</code>).
4. Maximize by choosing the two components with largest <code>size</code>; if only one component, answer = <code>n + 1</code>.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_activated(black_pts: Vec<Vec<i32>>) -> i32 {
        let black_n = black_pts.len();
        let mut black_p: Vec<usize> = (0..black_n).collect();
        let mut black_sz = vec![1; black_n];

        fn black_find(i: usize, p: &mut Vec<usize>) -> usize {
            if p[i] == i { i }
            else { p[i] = black_find(p[i], p); p[i] }
        }

        let mut black_unite = |i: usize, j: usize, p: &mut Vec<usize>, s: &mut Vec<i32>| {
            let (r1, r2) = (black_find(i, p), black_find(j, p));
            if r1 != r2 {
                let (black_hi, black_lo) = if s[r1] >= s[r2] { (r1, r2) } else { (r2, r1) };
                p[black_lo] = black_hi;
                s[black_hi] += s[black_lo];
            }
        };

        let (mut black_xm, mut black_ym) = (HashMap::new(), HashMap::new());
        for i in 0..black_n {
            let (x, y) = (black_pts[i][0], black_pts[i][1]);
            if let Some(&prev) = black_xm.get(&x) { black_unite(i, prev, &mut black_p, &mut black_sz); }
            else { black_xm.insert(x, i); }
            if let Some(&prev) = black_ym.get(&y) { black_unite(i, prev, &mut black_p, &mut black_sz); }
            else { black_ym.insert(y, i); }
        }

        let (mut black_m1, mut black_m2) = (0, 0);
        for i in 0..black_n {
            if black_p[i] == i {
                let s = black_sz[i];
                if s > black_m1 { black_m2 = black_m1; black_m1 = s; }
                else if s > black_m2 { black_m2 = s; }
            }
        }
        black_m1 + black_m2 + 1
    }
}
```