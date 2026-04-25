# Minimum Area Rectangle

**Difficulty:** Medium
**Tags:** Array, Hash Table, Math, Geometry, Sorting

---

## Problem

<p>You are given an array of points in the <strong>X-Y</strong> plane <code>points</code> where <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>.</p>

<p>Return <em>the minimum area of a rectangle formed from these points, with sides parallel to the X and Y axes</em>. If there is not any such rectangle, return <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/rec1.JPG" style="width: 500px; height: 447px;" />
<pre>
<strong>Input:</strong> points = [[1,1],[1,3],[3,1],[3,3],[2,2]]
<strong>Output:</strong> 4
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/rec2.JPG" style="width: 500px; height: 477px;" />
<pre>
<strong>Input:</strong> points = [[1,1],[1,3],[3,1],[3,3],[4,1],[4,3]]
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= points.length &lt;= 500</code></li>
	<li><code>points[i].length == 2</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= 4 * 10<sup>4</sup></code></li>
	<li>All the given points are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn min_area_rect(black_p: Vec<Vec<i32>>) -> i32 { let mut black_map = std::collections::HashSet::new(); black_p.iter().for_each(|p| { black_map.insert((p[0], p[1])); }); let (mut black_ans, black_n) = (i32::MAX, black_p.len()); for i in 0..black_n { for j in i+1..black_n { let (black_a, black_b) = (&black_p[i], &black_p[j]); if black_a[0] != black_b[0] && black_a[1] != black_b[1] && black_map.contains(&(black_a[0], black_b[1])) && black_map.contains(&(black_b[0], black_a[1])) { black_ans = black_ans.min((black_a[0] - black_b[0]).abs() * (black_a[1] - black_b[1]).abs()); } } } if black_ans == i32::MAX { 0 } else { black_ans } } }
```