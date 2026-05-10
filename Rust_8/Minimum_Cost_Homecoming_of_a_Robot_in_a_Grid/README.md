# Minimum Cost Homecoming of a Robot in a Grid

**Difficulty:** Medium
**Tags:** Array, Greedy

---

## Problem

<p>There is an <code>m x n</code> grid, where <code>(0, 0)</code> is the top-left cell and <code>(m - 1, n - 1)</code> is the bottom-right cell. You are given an integer array <code>startPos</code> where <code>startPos = [start<sub>row</sub>, start<sub>col</sub>]</code> indicates that <strong>initially</strong>, a <strong>robot</strong> is at the cell <code>(start<sub>row</sub>, start<sub>col</sub>)</code>. You are also given an integer array <code>homePos</code> where <code>homePos = [home<sub>row</sub>, home<sub>col</sub>]</code> indicates that its <strong>home</strong> is at the cell <code>(home<sub>row</sub>, home<sub>col</sub>)</code>.</p>

<p>The robot needs to go to its home. It can move one cell in four directions: <strong>left</strong>, <strong>right</strong>, <strong>up</strong>, or <strong>down</strong>, and it can not move outside the boundary. Every move incurs some cost. You are further given two <strong>0-indexed</strong> integer arrays: <code>rowCosts</code> of length <code>m</code> and <code>colCosts</code> of length <code>n</code>.</p>

<ul>
	<li>If the robot moves <strong>up</strong> or <strong>down</strong> into a cell whose <strong>row</strong> is <code>r</code>, then this move costs <code>rowCosts[r]</code>.</li>
	<li>If the robot moves <strong>left</strong> or <strong>right</strong> into a cell whose <strong>column</strong> is <code>c</code>, then this move costs <code>colCosts[c]</code>.</li>
</ul>

<p>Return <em>the <strong>minimum total cost</strong> for this robot to return home</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/10/11/eg-1.png" style="width: 282px; height: 217px;" />
<pre>
<strong>Input:</strong> startPos = [1, 0], homePos = [2, 3], rowCosts = [5, 4, 3], colCosts = [8, 2, 6, 7]
<strong>Output:</strong> 18
<strong>Explanation:</strong> One optimal path is that:
Starting from (1, 0)
-&gt; It goes down to (<u><strong>2</strong></u>, 0). This move costs rowCosts[2] = 3.
-&gt; It goes right to (2, <u><strong>1</strong></u>). This move costs colCosts[1] = 2.
-&gt; It goes right to (2, <u><strong>2</strong></u>). This move costs colCosts[2] = 6.
-&gt; It goes right to (2, <u><strong>3</strong></u>). This move costs colCosts[3] = 7.
The total cost is 3 + 2 + 6 + 7 = 18</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> startPos = [0, 0], homePos = [0, 0], rowCosts = [5], colCosts = [26]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The robot is already at its home. Since no moves occur, the total cost is 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == rowCosts.length</code></li>
	<li><code>n == colCosts.length</code></li>
	<li><code>1 &lt;= m, n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= rowCosts[r], colCosts[c] &lt;= 10<sup>4</sup></code></li>
	<li><code>startPos.length == 2</code></li>
	<li><code>homePos.length == 2</code></li>
	<li><code>0 &lt;= start<sub>row</sub>, home<sub>row</sub> &lt; m</code></li>
	<li><code>0 &lt;= start<sub>col</sub>, home<sub>col</sub> &lt; n</code></li>
</ul>


## Hints

1. Irrespective of what path the robot takes, it will have to traverse all the rows between startRow and homeRow and all the columns between startCol and homeCol.
2. Hence, making any other move other than traversing the required rows and columns will potentially incur more cost which can be avoided.

## Solution

```rust
impl Solution { pub fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 { let (mut black_res, black_r1, black_r2, black_c1, black_c2) = (0, start_pos[0], home_pos[0], start_pos[1], home_pos[1]); if black_r1 < black_r2 { for black_i in black_r1 + 1..=black_r2 { black_res += row_costs[black_i as usize]; } } else { for black_i in black_r2..black_r1 { black_res += row_costs[black_i as usize]; } } if black_c1 < black_c2 { for black_i in black_c1 + 1..=black_c2 { black_res += col_costs[black_i as usize]; } } else { for black_i in black_c2..black_c1 { black_res += col_costs[black_i as usize]; } } black_res } }
```