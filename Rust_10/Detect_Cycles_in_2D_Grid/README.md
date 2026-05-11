# Detect Cycles in 2D Grid

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Union-Find, Matrix

---

## Problem

<p>Given a 2D array of characters <code>grid</code> of size <code>m x n</code>, you need to find if there exists any cycle consisting of the <strong>same value</strong> in <code>grid</code>.</p>

<p>A cycle is a path of <strong>length 4 or more</strong> in the grid that starts and ends at the same cell. From a given cell, you can move to one of the cells adjacent to it - in one of the four directions (up, down, left, or right), if it has the <strong>same value</strong> of the current cell.</p>

<p>Also, you cannot move to the cell that you visited in your last move. For example, the cycle <code>(1, 1) -&gt; (1, 2) -&gt; (1, 1)</code> is invalid because from <code>(1, 2)</code> we visited <code>(1, 1)</code> which was the last visited cell.</p>

<p>Return <code>true</code> if any cycle of the same value exists in <code>grid</code>, otherwise, return <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/1.png" style="width: 231px; height: 152px;" /></strong></p>

<pre>
<strong>Input:</strong> grid = [[&quot;a&quot;,&quot;a&quot;,&quot;a&quot;,&quot;a&quot;],[&quot;a&quot;,&quot;b&quot;,&quot;b&quot;,&quot;a&quot;],[&quot;a&quot;,&quot;b&quot;,&quot;b&quot;,&quot;a&quot;],[&quot;a&quot;,&quot;a&quot;,&quot;a&quot;,&quot;a&quot;]]
<strong>Output:</strong> true
<strong>Explanation: </strong>There are two valid cycles shown in different colors in the image below:
<img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/11.png" style="width: 225px; height: 163px;" />
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/22.png" style="width: 236px; height: 154px;" /></strong></p>

<pre>
<strong>Input:</strong> grid = [[&quot;c&quot;,&quot;c&quot;,&quot;c&quot;,&quot;a&quot;],[&quot;c&quot;,&quot;d&quot;,&quot;c&quot;,&quot;c&quot;],[&quot;c&quot;,&quot;c&quot;,&quot;e&quot;,&quot;c&quot;],[&quot;f&quot;,&quot;c&quot;,&quot;c&quot;,&quot;c&quot;]]
<strong>Output:</strong> true
<strong>Explanation: </strong>There is only one valid cycle highlighted in the image below:
<img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/2.png" style="width: 229px; height: 157px;" />
</pre>

<p><strong class="example">Example 3:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/07/15/3.png" style="width: 183px; height: 120px;" /></strong></p>

<pre>
<strong>Input:</strong> grid = [[&quot;a&quot;,&quot;b&quot;,&quot;b&quot;],[&quot;b&quot;,&quot;z&quot;,&quot;b&quot;],[&quot;b&quot;,&quot;b&quot;,&quot;a&quot;]]
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == grid.length</code></li>
	<li><code>n == grid[i].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 500</code></li>
	<li><code>grid</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Keep track of the parent (previous position) to avoid considering an invalid path.
2. Use DFS or BFS and keep track of visited cells to see if there is a cycle.

## Solution

```rust
impl Solution { pub fn contains_cycle(black_g: Vec<Vec<char>>) -> bool { let (black_r, black_c) = (black_g.len(), black_g[0].len()); let mut black_f: Vec<usize> = (0..black_r*black_c).collect(); fn black_find(mut i: usize, f: &mut Vec<usize>) -> usize { while f[i] != i { f[i] = f[f[i]]; i = f[i]; } i } for i in 0..black_r { for j in 0..black_c { if i > 0 && black_g[i][j] == black_g[i-1][j] { let (r1, r2) = (black_find(i*black_c+j, &mut black_f), black_find((i-1)*black_c+j, &mut black_f)); if r1 == r2 { return true; } black_f[r1] = r2; } if j > 0 && black_g[i][j] == black_g[i][j-1] { let (r1, r2) = (black_find(i*black_c+j, &mut black_f), black_find(i*black_c+j-1, &mut black_f)); if r1 == r2 { return true; } black_f[r1] = r2; } } } false } }
```