# Number of Provinces

**Difficulty:** Medium
**Tags:** Depth-First Search, Breadth-First Search, Union-Find, Graph Theory

---

## Problem

<p>There are <code>n</code> cities. Some of them are connected, while some are not. If city <code>a</code> is connected directly with city <code>b</code>, and city <code>b</code> is connected directly with city <code>c</code>, then city <code>a</code> is connected indirectly with city <code>c</code>.</p>

<p>A <strong>province</strong> is a group of directly or indirectly connected cities and no other cities outside of the group.</p>

<p>You are given an <code>n x n</code> matrix <code>isConnected</code> where <code>isConnected[i][j] = 1</code> if the <code>i<sup>th</sup></code> city and the <code>j<sup>th</sup></code> city are directly connected, and <code>isConnected[i][j] = 0</code> otherwise.</p>

<p>Return <em>the total number of <strong>provinces</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph1.jpg" style="width: 222px; height: 142px;" />
<pre>
<strong>Input:</strong> isConnected = [[1,1,0],[1,1,0],[0,0,1]]
<strong>Output:</strong> 2
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/graph2.jpg" style="width: 222px; height: 142px;" />
<pre>
<strong>Input:</strong> isConnected = [[1,0,0],[0,1,0],[0,0,1]]
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 200</code></li>
	<li><code>n == isConnected.length</code></li>
	<li><code>n == isConnected[i].length</code></li>
	<li><code>isConnected[i][j]</code> is <code>1</code> or <code>0</code>.</li>
	<li><code>isConnected[i][i] == 1</code></li>
	<li><code>isConnected[i][j] == isConnected[j][i]</code></li>
</ul>



## Solution

```rust
impl Solution { pub fn find_circle_num(black_is_connected: Vec<Vec<i32>>) -> i32 { let mut black_vis = vec![false; black_is_connected.len()]; let mut black_ans = 0; fn dfs(i: usize, vis: &mut Vec<bool>, conn: &Vec<Vec<i32>>) { vis[i] = true; for j in 0..conn.len() { if conn[i][j] == 1 && !vis[j] { dfs(j, vis, conn); } } } for i in 0..black_is_connected.len() { if !black_vis[i] { black_ans += 1; dfs(i, &mut black_vis, &black_is_connected); } } black_ans } }
```