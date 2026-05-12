# Find Maximum Area of a Triangle

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Greedy, Geometry, Enumeration

---

## Problem

<p>You are given a 2D array <code>coords</code> of size <code>n x 2</code>, representing the coordinates of <code>n</code> points in an infinite Cartesian plane.</p>

<p>Find <strong>twice</strong> the <strong>maximum</strong> area of a triangle with its corners at <em>any</em> three elements from <code>coords</code>, such that at least one side of this triangle is <strong>parallel</strong> to the x-axis or y-axis. Formally, if the maximum area of such a triangle is <code>A</code>, return <code>2 * A</code>.</p>

<p>If no such triangle exists, return -1.</p>

<p><strong>Note</strong> that a triangle <em>cannot</em> have zero area.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">coords = [[1,1],[1,2],[3,2],[3,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><img src="https://assets.leetcode.com/uploads/2025/04/19/image-20250420010047-1.png" style="width: 300px; height: 289px;" /></p>

<p>The triangle shown in the image has a base 1 and height 2. Hence its area is <code>1/2 * base * height = 1</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">coords = [[1,1],[2,2],[3,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only possible triangle has corners <code>(1, 1)</code>, <code>(2, 2)</code>, and <code>(3, 3)</code>. None of its sides are parallel to the x-axis or the y-axis.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == coords.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= coords[i][0], coords[i][1] &lt;= 10<sup>6</sup></code></li>
	<li>All <code>coords[i]</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. <code>area * 2 = base * height</code>
2. Let the base be parallel to the x‑axis or y‑axis
3. Sort to find the maximum base for each fixed <code>x</code> (or <code>y</code>), then the maximum height comes from the extreme values of the other coordinate.

## Solution

```rust
impl Solution { pub fn max_area(black_coords: Vec<Vec<i32>>) -> i64 { use std::collections::HashMap; let (mut black_mpx, mut black_mpy) = (HashMap::new(), HashMap::new()); let (mut black_xm, mut black_xn, mut black_ym, mut black_yn) = (i64::MIN, i64::MAX, i64::MIN, i64::MAX); for black_it in &black_coords { let (black_x, black_y) = (black_it[0] as i64, black_it[1] as i64); let black_ex = black_mpx.entry(black_x).or_insert((i64::MAX, i64::MIN)); black_ex.0 = black_ex.0.min(black_y); black_ex.1 = black_ex.1.max(black_y); let black_ey = black_mpy.entry(black_y).or_insert((i64::MAX, i64::MIN)); black_ey.0 = black_ey.0.min(black_x); black_ey.1 = black_ey.1.max(black_x); black_xm = black_xm.max(black_x); black_xn = black_xn.min(black_x); black_ym = black_ym.max(black_y); black_yn = black_yn.min(black_y); } let mut black_res = 0i64; for (black_x, black_p) in black_mpx { let black_b = (black_p.1 - black_p.0).abs(); let black_h = (black_x - black_xm).abs().max((black_x - black_xn).abs()); black_res = black_res.max(black_b * black_h); } for (black_y, black_p) in black_mpy { let black_b = (black_p.1 - black_p.0).abs(); let black_h = (black_y - black_ym).abs().max((black_y - black_yn).abs()); black_res = black_res.max(black_b * black_h); } if black_res == 0 { -1 } else { black_res } } }
```