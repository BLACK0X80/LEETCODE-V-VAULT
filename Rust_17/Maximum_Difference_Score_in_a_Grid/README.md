# Maximum Difference Score in a Grid

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Matrix

---

## Problem

<p>You are given an <code>m x n</code> matrix <code>grid</code> consisting of <strong>positive</strong> integers. You can move from a cell in the matrix to <strong>any</strong> other cell that is either to the bottom or to the right (not necessarily adjacent). The score of a move from a cell with the value <code>c1</code> to a cell with the value <code>c2</code> is <code>c2 - c1</code>.<!-- notionvc: 8819ca04-8606-4ecf-815b-fb77bc63b851 --></p>

<p>You can start at <strong>any</strong> cell, and you have to make <strong>at least</strong> one move.</p>

<p>Return the <strong>maximum</strong> total score you can achieve.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/03/14/grid1.png" style="width: 240px; height: 240px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[9,5,7,3],[8,9,6,1],[6,7,14,3],[2,5,3,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">9</span></p>

<p><strong>Explanation:</strong> We start at the cell <code>(0, 1)</code>, and we perform the following moves:<br />
- Move from the cell <code>(0, 1)</code> to <code>(2, 1)</code> with a score of <code>7 - 5 = 2</code>.<br />
- Move from the cell <code>(2, 1)</code> to <code>(2, 2)</code> with a score of <code>14 - 7 = 7</code>.<br />
The total score is <code>2 + 7 = 9</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/04/08/moregridsdrawio-1.png" style="width: 180px; height: 116px;" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">grid = [[4,3,2],[3,2,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong> We start at the cell <code>(0, 0)</code>, and we perform one move: <code>(0, 0)</code> to <code>(0, 1)</code>. The score is <code>3 - 4 = -1</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>2 &lt;= m, n &lt;= 1000</code></li>
	<li><code>4 &lt;= m * n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= grid[i][j] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Any path from a cell <code>(x1, y1)</code> to another cell <code>(x2, y2)</code> will always have a score of <code>grid[x2][y2] - grid[x1][y1]</code>.
2. Let’s say we fix the starting cell <code>(x1, y1)</code>, how to the find a cell <code>(x2, y2)</code> such that the value <code>grid[x2][y2] - grid[x1][y1]</code> is the maximum possible?

## Solution

```rust
impl Solution { pub fn max_score(grid: Vec<Vec<i32>>) -> i32 { let (m, n) = (grid.len(), grid[0].len()); let (mut black_min, mut black_ans) = (vec![vec![1000000; n]; m], -1000000); for i in 0..m { for j in 0..n { let prev_min = match (i > 0, j > 0) { (true, true) => black_min[i-1][j].min(black_min[i][j-1]), (true, false) => black_min[i-1][j], (false, true) => black_min[i][j-1], _ => 1000000 }; black_ans = black_ans.max(grid[i][j] - prev_min); black_min[i][j] = grid[i][j].min(prev_min); } } black_ans } }
```