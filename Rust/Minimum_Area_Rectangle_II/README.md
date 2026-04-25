# Minimum Area Rectangle II

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Geometry

---

## Problem

<p>You are given an array of points in the <strong>X-Y</strong> plane <code>points</code> where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>.</p>

<p>Return <em>the minimum area of any rectangle formed from these points, with sides <strong>not necessarily parallel</strong> to the X and Y axes</em>. If there is not any such rectangle, return <code>0</code>.</p>

<p>Answers within <code>10<sup>-5</sup></code> of the actual answer will be accepted.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/21/1a.png" style="width: 398px; height: 400px;" />
<pre>
<strong>Input:</strong> points = [[1,2],[2,1],[1,0],[0,1]]
<strong>Output:</strong> 2.00000
<strong>Explanation:</strong> The minimum area rectangle occurs at [1,2],[2,1],[1,0],[0,1], with an area of 2.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/22/2.png" style="width: 400px; height: 251px;" />
<pre>
<strong>Input:</strong> points = [[0,1],[2,1],[1,1],[1,0],[2,0]]
<strong>Output:</strong> 1.00000
<strong>Explanation:</strong> The minimum area rectangle occurs at [1,0],[1,1],[2,1],[2,0], with an area of 1.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/22/3.png" style="width: 383px; height: 400px;" />
<pre>
<strong>Input:</strong> points = [[0,3],[1,2],[3,1],[1,3],[2,1]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no possible rectangle to form from these points.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= points.length &lt;= 50</code></li>
	<li><code>points[i].length == 2</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 4 * 10<sup>4</sup></code></li>
	<li>All the given points are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn min_area_free_rect(black_p: Vec<Vec<i32>>) -> f64 { let black_pts: std::collections::HashSet<_> = black_p.iter().map(|v| (v[0], v[1])).collect(); let (mut black_ans, black_n) = (f64::MAX, black_p.len()); for i in 0..black_n { for j in 0..black_n { for k in j+1..black_n { if i == j || i == k { continue; } let (black_p1, black_p2, black_p3) = (&black_p[i], &black_p[j], &black_p[k]); if (black_p2[0]-black_p1[0])*(black_p3[0]-black_p1[0]) + (black_p2[1]-black_p1[1])*(black_p3[1]-black_p1[1]) == 0 { let black_p4 = (black_p2[0]+black_p3[0]-black_p1[0], black_p2[1]+black_p3[1]-black_p1[1]); if black_pts.contains(&black_p4) { let black_area = (((black_p2[0]-black_p1[0]).pow(2) + (black_p2[1]-black_p1[1]).pow(2)) as f64).sqrt() * (((black_p3[0]-black_p1[0]).pow(2) + (black_p3[1]-black_p1[1]).pow(2)) as f64).sqrt(); black_ans = black_ans.min(black_area); } } } } } if black_ans == f64::MAX { 0.0 } else { black_ans } } }
```