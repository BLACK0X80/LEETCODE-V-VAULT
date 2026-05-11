# Minimum Moves to Reach Target with Rotations

**Difficulty:** Hard
**Tags:** Array, Breadth-First Search, Matrix

---

## Problem

<p>In an&nbsp;<code>n*n</code>&nbsp;grid, there is a snake that spans 2 cells and starts moving from the top left corner at <code>(0, 0)</code> and <code>(0, 1)</code>. The grid has empty cells represented by zeros and blocked cells represented by ones. The snake wants to reach the lower right corner at&nbsp;<code>(n-1, n-2)</code>&nbsp;and&nbsp;<code>(n-1, n-1)</code>.</p>

<p>In one move the snake can:</p>

<ul>
	<li>Move one cell to the right&nbsp;if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.</li>
	<li>Move down one cell&nbsp;if there are no blocked cells there. This move keeps the horizontal/vertical position of the snake as it is.</li>
	<li>Rotate clockwise if it&#39;s in a horizontal position and the two cells under it are both empty. In that case the snake moves from&nbsp;<code>(r, c)</code>&nbsp;and&nbsp;<code>(r, c+1)</code>&nbsp;to&nbsp;<code>(r, c)</code>&nbsp;and&nbsp;<code>(r+1, c)</code>.<br />
	<img alt="" src="https://assets.leetcode.com/uploads/2019/09/24/image-2.png" style="width: 300px; height: 134px;" /></li>
	<li>Rotate counterclockwise&nbsp;if it&#39;s in a vertical position and the two cells to its right are both empty. In that case the snake moves from&nbsp;<code>(r, c)</code>&nbsp;and&nbsp;<code>(r+1, c)</code>&nbsp;to&nbsp;<code>(r, c)</code>&nbsp;and&nbsp;<code>(r, c+1)</code>.<br />
	<img alt="" src="https://assets.leetcode.com/uploads/2019/09/24/image-1.png" style="width: 300px; height: 121px;" /></li>
</ul>

<p>Return the minimum number of moves to reach the target.</p>

<p>If there is no way to reach the target, return&nbsp;<code>-1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2019/09/24/image.png" style="width: 400px; height: 439px;" /></strong></p>

<pre>
<strong>Input:</strong> grid = [[0,0,0,0,0,1],
               [1,1,0,0,1,0],
&nbsp;              [0,0,0,0,1,1],
&nbsp;              [0,0,1,0,1,0],
&nbsp;              [0,1,1,0,0,0],
&nbsp;              [0,1,1,0,0,0]]
<strong>Output:</strong> 11
<strong>Explanation:
</strong>One possible solution is [right, right, rotate clockwise, right, down, down, down, down, rotate counterclockwise, right, down].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> grid = [[0,0,1,1,1,1],
&nbsp;              [0,0,0,0,1,1],
&nbsp;              [1,1,0,0,0,1],
&nbsp;              [1,1,1,0,0,1],
&nbsp;              [1,1,1,0,0,1],
&nbsp;              [1,1,1,0,0,0]]
<strong>Output:</strong> 9
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 100</code></li>
	<li><code>0 &lt;= grid[i][j] &lt;= 1</code></li>
	<li>It is guaranteed that the snake starts at empty cells.</li>
</ul>


## Hints

1. Use BFS to find the answer.
2. The state of the BFS is the position (x, y) along with a binary value that specifies if the position is horizontal or vertical.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut vis = vec![vec![vec![false;2];n];n];
        let mut q = VecDeque::new();
        q.push_back((0usize,0usize,0usize,0i32));
        vis[0][0][0]=true;
        while let Some((r,c,dir,dist)) = q.pop_front() {
            if r==n-1 && c==n-2 && dir==0 { return dist; }
            if dir==0 {
                if c+2<n && grid[r][c+2]==0 && !vis[r][c+1][0] { vis[r][c+1][0]=true; q.push_back((r,c+1,0,dist+1)); }
                if r+1<n && grid[r+1][c]==0 && grid[r+1][c+1]==0 {
                    if !vis[r+1][c][0] { vis[r+1][c][0]=true; q.push_back((r+1,c,0,dist+1)); }
                    if !vis[r][c][1] { vis[r][c][1]=true; q.push_back((r,c,1,dist+1)); }
                }
            } else {
                if r+2<n && grid[r+2][c]==0 && !vis[r+1][c][1] { vis[r+1][c][1]=true; q.push_back((r+1,c,1,dist+1)); }
                if c+1<n && grid[r][c+1]==0 && grid[r+1][c+1]==0 {
                    if !vis[r][c+1][1] { vis[r][c+1][1]=true; q.push_back((r,c+1,1,dist+1)); }
                    if !vis[r][c][0] { vis[r][c][0]=true; q.push_back((r,c,0,dist+1)); }
                }
            }
        }
        -1
    }
}
```