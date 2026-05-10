# Find the Minimum Area to Cover All Ones I

**Difficulty:** Medium
**Tags:** Array, Matrix

---

## Problem

<p>You are given a 2D <strong>binary</strong> array <code>grid</code>. Find a rectangle with horizontal and vertical sides with the<strong> smallest</strong> area, such that all the 1&#39;s in <code>grid</code> lie inside this rectangle.</p>

<p>Return the <strong>minimum</strong> possible area of the rectangle.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[0,1,0],[1,0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/05/08/examplerect0.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 279px; height: 198px;" /></p>

<p>The smallest rectangle has a height of 2 and a width of 3, so it has an area of <code>2 * 3 = 6</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[1,0],[0,0]]</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/05/08/examplerect1.png" style="padding: 10px; background: rgb(255, 255, 255); border-radius: 0.5rem; width: 204px; height: 201px;" /></p>

<p>The smallest rectangle has both height and width 1, so its area is <code>1 * 1 = 1</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= grid.length, grid[i].length &lt;= 1000</code></li>
	<li><code>grid[i][j]</code> is either 0 or 1.</li>
	<li>The input is generated such that there is at least one 1 in <code>grid</code>.</li>
</ul>


## Hints

1. Find the minimum and maximum coordinates of a cell with a value of 1 in both directions.

## Solution

```rust
impl Solution { pub fn minimum_area(black_g: Vec<Vec<i32>>) -> i32 { let (mut black_r1, mut black_r2, mut black_c1, mut black_c2) = (1001, -1, 1001, -1); for i in 0..black_g.len() { for j in 0..black_g[0].len() { if black_g[i][j] == 1 { black_r1 = black_r1.min(i as i32); black_r2 = black_r2.max(i as i32); black_c1 = black_c1.min(j as i32); black_c2 = black_c2.max(j as i32); } } } if black_r2 == -1 { 0 } else { (black_r2 - black_r1 + 1) * (black_c2 - black_c1 + 1) } } }
```