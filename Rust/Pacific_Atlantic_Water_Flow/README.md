# Pacific Atlantic Water Flow

**Difficulty:** Medium
**Tags:** Array, Depth-First Search, Breadth-First Search, Matrix

---

## Problem

<p>There is an <code>m x n</code> rectangular island that borders both the <strong>Pacific Ocean</strong> and <strong>Atlantic Ocean</strong>. The <strong>Pacific Ocean</strong> touches the island&#39;s left and top edges, and the <strong>Atlantic Ocean</strong> touches the island&#39;s right and bottom edges.</p>

<p>The island is partitioned into a grid of square cells. You are given an <code>m x n</code> integer matrix <code>heights</code> where <code>heights[r][c]</code> represents the <strong>height above sea level</strong> of the cell at coordinate <code>(r, c)</code>.</p>

<p>The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell&#39;s height is <strong>less than or equal to</strong> the current cell&#39;s height. Water can flow from any cell adjacent to an ocean into the ocean.</p>

<p>Return <em>a <strong>2D list</strong> of grid coordinates </em><code>result</code><em> where </em><code>result[i] = [r<sub>i</sub>, c<sub>i</sub>]</code><em> denotes that rain water can flow from cell </em><code>(r<sub>i</sub>, c<sub>i</sub>)</code><em> to <strong>both</strong> the Pacific and Atlantic oceans</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/waterflow-grid.jpg" style="width: 400px; height: 400px;" />
<pre>
<strong>Input:</strong> heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
<strong>Output:</strong> [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
<strong>Explanation:</strong> The following cells can flow to the Pacific and Atlantic oceans, as shown below:
[0,4]: [0,4] -&gt; Pacific Ocean 
&nbsp;      [0,4] -&gt; Atlantic Ocean
[1,3]: [1,3] -&gt; [0,3] -&gt; Pacific Ocean 
&nbsp;      [1,3] -&gt; [1,4] -&gt; Atlantic Ocean
[1,4]: [1,4] -&gt; [1,3] -&gt; [0,3] -&gt; Pacific Ocean 
&nbsp;      [1,4] -&gt; Atlantic Ocean
[2,2]: [2,2] -&gt; [1,2] -&gt; [0,2] -&gt; Pacific Ocean 
&nbsp;      [2,2] -&gt; [2,3] -&gt; [2,4] -&gt; Atlantic Ocean
[3,0]: [3,0] -&gt; Pacific Ocean 
&nbsp;      [3,0] -&gt; [4,0] -&gt; Atlantic Ocean
[3,1]: [3,1] -&gt; [3,0] -&gt; Pacific Ocean 
&nbsp;      [3,1] -&gt; [4,1] -&gt; Atlantic Ocean
[4,0]: [4,0] -&gt; Pacific Ocean 
       [4,0] -&gt; Atlantic Ocean
Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> heights = [[1]]
<strong>Output:</strong> [[0,0]]
<strong>Explanation:</strong> The water can flow from the only cell to the Pacific and Atlantic oceans.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>m == heights.length</code></li>
	<li><code>n == heights[r].length</code></li>
	<li><code>1 &lt;= m, n &lt;= 200</code></li>
	<li><code>0 &lt;= heights[r][c] &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn pacific_atlantic(black_h: Vec<Vec<i32>>) -> Vec<Vec<i32>> { let (black_m, black_n) = (black_h.len(), black_h[0].len()); let mut black_flow = vec![vec![0; black_n]; black_m]; fn black_dfs(r: usize, c: usize, prev: i32, ocean: i32, h: &Vec<Vec<i32>>, f: &mut Vec<Vec<i32>>, m: usize, n: usize) { if r >= m || c >= n || h[r][c] < prev || (f[r][c] & ocean) != 0 { return; } f[r][c] |= ocean; [[0,1],[0,-1],[1,0],[-1,0]].iter().for_each(|d| black_dfs((r as i32 + d[0]) as usize, (c as i32 + d[1]) as usize, h[r][c], ocean, h, f, m, n)); } for black_i in 0..black_m { black_dfs(black_i, 0, 0, 1, &black_h, &mut black_flow, black_m, black_n); black_dfs(black_i, black_n - 1, 0, 2, &black_h, &mut black_flow, black_m, black_n); } for black_j in 0..black_n { black_dfs(0, black_j, 0, 1, &black_h, &mut black_flow, black_m, black_n); black_dfs(black_m - 1, black_j, 0, 2, &black_h, &mut black_flow, black_m, black_n); } let mut black_res = vec![]; for black_r in 0..black_m { for black_c in 0..black_n { if black_flow[black_r][black_c] == 3 { black_res.push(vec![black_r as i32, black_c as i32]); } } } black_res } }
```