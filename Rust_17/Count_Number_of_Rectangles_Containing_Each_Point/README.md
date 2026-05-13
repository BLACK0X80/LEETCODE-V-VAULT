# Count Number of Rectangles Containing Each Point

**Difficulty:** Medium
**Tags:** Array, Hash Table, Binary Search, Binary Indexed Tree, Sorting

---

## Problem

<p>You are given a 2D integer array <code>rectangles</code> where <code>rectangles[i] = [l<sub>i</sub>, h<sub>i</sub>]</code> indicates that <code>i<sup>th</sup></code> rectangle has a length of <code>l<sub>i</sub></code> and a height of <code>h<sub>i</sub></code>. You are also given a 2D integer array <code>points</code> where <code>points[j] = [x<sub>j</sub>, y<sub>j</sub>]</code> is a point with coordinates <code>(x<sub>j</sub>, y<sub>j</sub>)</code>.</p>

<p>The <code>i<sup>th</sup></code> rectangle has its <strong>bottom-left corner</strong> point at the coordinates <code>(0, 0)</code> and its <strong>top-right corner</strong> point at <code>(l<sub>i</sub>, h<sub>i</sub>)</code>.</p>

<p>Return<em> an integer array </em><code>count</code><em> of length </em><code>points.length</code><em> where </em><code>count[j]</code><em> is the number of rectangles that <strong>contain</strong> the </em><code>j<sup>th</sup></code><em> point.</em></p>

<p>The <code>i<sup>th</sup></code> rectangle <strong>contains</strong> the <code>j<sup>th</sup></code> point if <code>0 &lt;= x<sub>j</sub> &lt;= l<sub>i</sub></code> and <code>0 &lt;= y<sub>j</sub> &lt;= h<sub>i</sub></code>. Note that points that lie on the <strong>edges</strong> of a rectangle are also considered to be contained by that rectangle.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/example1.png" style="width: 300px; height: 509px;" />
<pre>
<strong>Input:</strong> rectangles = [[1,2],[2,3],[2,5]], points = [[2,1],[1,4]]
<strong>Output:</strong> [2,1]
<strong>Explanation:</strong> 
The first rectangle contains no points.
The second rectangle contains only the point (2, 1).
The third rectangle contains the points (2, 1) and (1, 4).
The number of rectangles that contain the point (2, 1) is 2.
The number of rectangles that contain the point (1, 4) is 1.
Therefore, we return [2, 1].
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/example2.png" style="width: 300px; height: 312px;" />
<pre>
<strong>Input:</strong> rectangles = [[1,1],[2,2],[3,3]], points = [[1,3],[1,1]]
<strong>Output:</strong> [1,3]
<strong>Explanation:
</strong>The first rectangle contains only the point (1, 1).
The second rectangle contains only the point (1, 1).
The third rectangle contains the points (1, 3) and (1, 1).
The number of rectangles that contain the point (1, 3) is 1.
The number of rectangles that contain the point (1, 1) is 3.
Therefore, we return [1, 3].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= rectangles.length, points.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>rectangles[i].length == points[j].length == 2</code></li>
	<li><code>1 &lt;= l<sub>i</sub>, x<sub>j</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= h<sub>i</sub>, y<sub>j</sub> &lt;= 100</code></li>
	<li>All the <code>rectangles</code> are <strong>unique</strong>.</li>
	<li>All the <code>points</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. The heights of the rectangles and the y-coordinates of the points are only at most 100, so for each point, we can iterate over the possible heights of the rectangles that contain a given point.
2. For a given point and height, can we efficiently count how many rectangles with that height contain our point?
3. Sort the rectangles at each height and use binary search.

## Solution

```rust
impl Solution { pub fn count_rectangles(black_rects: Vec<Vec<i32>>, black_pts: Vec<Vec<i32>>) -> Vec<i32> { let mut black_map = vec![vec![]; 101]; for black_r in black_rects { black_map[black_r[1] as usize].push(black_r[0]); } for black_h in 1..101 { black_map[black_h].sort_unstable(); } black_pts.into_iter().map(|black_p| { (black_p[1] as usize..101).map(|black_h| (black_map[black_h].len() - black_map[black_h].partition_point(|&black_l| black_l < black_p[0])) as i32).sum() }).collect() } }
```