# Shortest Cycle in a Graph

**Difficulty:** Hard
**Tags:** Breadth-First Search, Graph Theory

---

## Problem

<p>There is a <strong>bi-directional </strong>graph with <code>n</code> vertices, where each vertex is labeled from <code>0</code> to <code>n - 1</code>. The edges in the graph are represented by a given 2D integer array <code>edges</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> denotes an edge between vertex <code>u<sub>i</sub></code> and vertex <code>v<sub>i</sub></code>. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.</p>

<p>Return <em>the length of the <strong>shortest </strong>cycle in the graph</em>. If no cycle exists, return <code>-1</code>.</p>

<p>A cycle is a path that starts and ends at the same node, and each edge in the path is used only once.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/01/04/cropped.png" style="width: 387px; height: 331px;" />
<pre>
<strong>Input:</strong> n = 7, edges = [[0,1],[1,2],[2,0],[3,4],[4,5],[5,6],[6,3]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The cycle with the smallest length is : 0 -&gt; 1 -&gt; 2 -&gt; 0 
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/01/04/croppedagin.png" style="width: 307px; height: 307px;" />
<pre>
<strong>Input:</strong> n = 4, edges = [[0,1],[0,2]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There are no cycles in this graph.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 1000</code></li>
	<li><code>1 &lt;= edges.length &lt;= 1000</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; n</code></li>
	<li><code>u<sub>i</sub> != v<sub>i</sub></code></li>
	<li>There are no repeated edges.</li>
</ul>


## Hints

1. How can BFS be used?
2. For each vertex u, calculate the length of the shortest cycle that contains vertex u using BFS

## Solution

```rust
use std::collections::VecDeque;
impl Solution {
    pub fn find_shortest_cycle(black_a: i32, black_b: Vec<Vec<i32>>) -> i32 {
        let black_n = black_a as usize;
        let mut black_c = vec![vec![]; black_n];
        for black_d in black_b { black_c[black_d[0] as usize].push(black_d[1] as usize); black_c[black_d[1] as usize].push(black_d[0] as usize); }
        let mut black_e = 1001;
        for black_f in 0..black_n {
            let mut black_g = vec![-1; black_n];
            let mut black_h = vec![-1; black_n];
            let mut black_i = VecDeque::new();
            black_g[black_f] = 0;
            black_i.push_back(black_f);
            while let Some(black_j) = black_i.pop_front() {
                for &black_k in &black_c[black_j] {
                    if black_g[black_k] == -1 {
                        black_g[black_k] = black_g[black_j] + 1;
                        black_h[black_k] = black_j as i32;
                        black_i.push_back(black_k);
                    } else if black_h[black_j] != black_k as i32 {
                        black_e = black_e.min(black_g[black_j] + black_g[black_k] + 1);
                    }
                }
            }
        }
        if black_e > 1000 { -1 } else { black_e }
    }
}
```