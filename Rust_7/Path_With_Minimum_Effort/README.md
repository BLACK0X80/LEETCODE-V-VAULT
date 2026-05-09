# Path With Minimum Effort

**Difficulty:** Medium
**Tags:** Array, Binary Search, Depth-First Search, Breadth-First Search, Union-Find, Heap (Priority Queue), Matrix

---

## Problem

<p>You are a hiker preparing for an upcoming hike. You are given <code>heights</code>, a 2D array of size <code>rows x columns</code>, where <code>heights[row][col]</code> represents the height of cell <code>(row, col)</code>. You are situated in the top-left cell, <code>(0, 0)</code>, and you hope to travel to the bottom-right cell, <code>(rows-1, columns-1)</code> (i.e.,&nbsp;<strong>0-indexed</strong>). You can move <strong>up</strong>, <strong>down</strong>, <strong>left</strong>, or <strong>right</strong>, and you wish to find a route that requires the minimum <strong>effort</strong>.</p>

<p>A route&#39;s <strong>effort</strong> is the <strong>maximum absolute difference</strong><strong> </strong>in heights between two consecutive cells of the route.</p>

<p>Return <em>the minimum <strong>effort</strong> required to travel from the top-left cell to the bottom-right cell.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex1.png" style="width: 300px; height: 300px;" /></p>

<pre>
<strong>Input:</strong> heights = [[1,2,2],[3,8,2],[5,3,5]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The route of [1,3,5,3,5] has a maximum absolute difference of 2 in consecutive cells.
This is better than the route of [1,2,2,2,5], where the maximum absolute difference is 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex2.png" style="width: 300px; height: 300px;" /></p>

<pre>
<strong>Input:</strong> heights = [[1,2,3],[3,8,4],[5,3,5]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The route of [1,2,3,4,5] has a maximum absolute difference of 1 in consecutive cells, which is better than route [1,3,5,3,5].
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/04/ex3.png" style="width: 300px; height: 300px;" />
<pre>
<strong>Input:</strong> heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> This route does not require any effort.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>rows == heights.length</code></li>
	<li><code>columns == heights[i].length</code></li>
	<li><code>1 &lt;= rows, columns &lt;= 100</code></li>
	<li><code>1 &lt;= heights[i][j] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Consider the grid as a graph, where adjacent cells have an edge with cost of the difference between the cells.
2. If you are given threshold k, check if it is possible to go from (0, 0) to (n-1, m-1) using only edges of ≤ k cost.
3. Binary search the k value.

## Solution

```rust
impl Solution { pub fn minimum_effort_path(black_h: Vec<Vec<i32>>) -> i32 { let (black_r, black_c) = (black_h.len(), black_h[0].len()); let mut black_e = vec![]; for i in 0..black_r { for j in 0..black_c { if i + 1 < black_r { black_e.push(((black_h[i][j] - black_h[i+1][j]).abs(), i*black_c+j, (i+1)*black_c+j)); } if j + 1 < black_c { black_e.push(((black_h[i][j] - black_h[i][j+1]).abs(), i*black_c+j, i*black_c+j+1)); } } } black_e.sort_unstable(); let mut black_f: Vec<usize> = (0..black_r*black_c).collect(); fn black_find(mut i: usize, f: &mut Vec<usize>) -> usize { while f[i] != i { f[i] = f[f[i]]; i = f[i]; } i } for (w, u, v) in black_e { let (r1, r2) = (black_find(u, &mut black_f), black_find(v, &mut black_f)); if r1 != r2 { black_f[r1] = r2; } if black_find(0, &mut black_f) == black_find(black_r*black_c-1, &mut black_f) { return w; } } 0 } }
```