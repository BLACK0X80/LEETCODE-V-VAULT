# Fill a Special Grid

**Difficulty:** Medium
**Tags:** Array, Divide and Conquer, Matrix

---

## Problem

<p>You are given a non-negative integer <code><font face="monospace">n</font></code> representing a <code>2<sup>n</sup> x 2<sup>n</sup></code> grid. You must fill the grid with integers from 0 to <code>2<sup>2n</sup> - 1</code> to make it <strong>special</strong>. A grid is <strong>special</strong> if it satisfies <strong>all</strong> the following conditions:</p>

<ul>
	<li>All numbers in the top-right quadrant are smaller than those in the bottom-right quadrant.</li>
	<li>All numbers in the bottom-right quadrant are smaller than those in the bottom-left quadrant.</li>
	<li>All numbers in the bottom-left quadrant are smaller than those in the top-left quadrant.</li>
	<li>Each of its quadrants is also a special grid.</li>
</ul>

<p>Return the <strong>special</strong> <code>2<sup>n</sup> x 2<sup>n</sup></code> grid.</p>

<p><strong>Note</strong>: Any 1x1 grid is special.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">[[0]]</span></p>

<p><strong>Explanation:</strong></p>

<p>The only number that can be placed is 0, and there is only one possible position in the grid.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">[[3,0],[2,1]]</span></p>

<p><strong>Explanation:</strong></p>

<p>The numbers in each quadrant are:</p>

<ul>
	<li>Top-right: 0</li>
	<li>Bottom-right: 1</li>
	<li>Bottom-left: 2</li>
	<li>Top-left: 3</li>
</ul>

<p>Since <code>0 &lt; 1 &lt; 2 &lt; 3</code>, this satisfies the given constraints.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[[15,12,3,0],[14,13,2,1],[11,8,7,4],[10,9,6,5]]</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2025/03/05/4123example3p1drawio.png" style="width: 161px; height: 161px;" /></p>

<p>The numbers in each quadrant are:</p>

<ul>
	<li>Top-right: 3, 0, 2, 1</li>
	<li>Bottom-right: 7, 4, 6, 5</li>
	<li>Bottom-left: 11, 8, 10, 9</li>
	<li>Top-left: 15, 12, 14, 13</li>
	<li><code>max(3, 0, 2, 1) &lt; min(7, 4, 6, 5)</code></li>
	<li><code>max(7, 4, 6, 5) &lt; min(11, 8, 10, 9)</code></li>
	<li><code>max(11, 8, 10, 9) &lt; min(15, 12, 14, 13)</code></li>
</ul>

<p>This satisfies the first three requirements. Additionally, each quadrant is also a special grid. Thus, this is a special grid.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 10</code></li>
</ul>


## Hints

1. Solve the problem recursively.

## Solution

```rust
impl Solution { pub fn special_grid(n: i32) -> Vec<Vec<i32>> { let black_sz = 1 << n; let mut black_res = vec![vec![0; black_sz]; black_sz]; fn solve(r: usize, c: usize, s: usize, black_v: i32, black_res: &mut Vec<Vec<i32>>) { if s == 1 { black_res[r][c] = black_v; return; } let black_ns = s / 2; let black_area = (black_ns * black_ns) as i32; solve(r, c + black_ns, black_ns, black_v, black_res); solve(r + black_ns, c + black_ns, black_ns, black_v + black_area, black_res); solve(r + black_ns, c, black_ns, black_v + 2 * black_area, black_res); solve(r, c, black_ns, black_v + 3 * black_area, black_res); } solve(0, 0, black_sz, 0, &mut black_res); black_res } }
```