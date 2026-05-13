# Add Edges to Make Degrees of All Nodes Even

**Difficulty:** Hard
**Tags:** Hash Table, Graph Theory

---

## Problem

<p>There is an <strong>undirected</strong> graph consisting of <code>n</code> nodes numbered from <code>1</code> to <code>n</code>. You are given the integer <code>n</code> and a <strong>2D</strong> array <code>edges</code> where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. The graph can be disconnected.</p>

<p>You can add <strong>at most</strong> two additional edges (possibly none) to this graph so that there are no repeated edges and no self-loops.</p>

<p>Return <code>true</code><em> if it is possible to make the degree of each node in the graph even, otherwise return </em><code>false</code><em>.</em></p>

<p>The degree of a node is the number of edges connected to it.</p>

<p>&nbsp;</p>
<p><strong>Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/10/26/agraphdrawio.png" style="width: 500px; height: 190px;" />
<pre>
<strong>Input:</strong> n = 5, edges = [[1,2],[2,3],[3,4],[4,2],[1,4],[2,5]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The above diagram shows a valid way of adding an edge.
Every node in the resulting graph is connected to an even number of edges.
</pre>

<p><strong>Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/10/26/aagraphdrawio.png" style="width: 400px; height: 120px;" />
<pre>
<strong>Input:</strong> n = 4, edges = [[1,2],[3,4]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The above diagram shows a valid way of adding two edges.</pre>

<p><strong>Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/10/26/aaagraphdrawio.png" style="width: 150px; height: 158px;" />
<pre>
<strong>Input:</strong> n = 4, edges = [[1,2],[1,3],[1,4]]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is not possible to obtain a valid graph with adding at most 2 edges.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= edges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>1 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt;= n</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li>There are no repeated edges.</li>
</ul>


## Hints

1. Notice that each edge that we add changes the degree of exactly 2 nodes.
2. The number of nodes with an odd degree in the original graph should be either 0, 2, or 4. Try to work on each of these cases.

## Solution

```rust
use std::collections::HashSet;

impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut adj = vec![HashSet::new(); n as usize + 1];
        let mut deg = vec![0; n as usize + 1];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u].insert(v);
            adj[v].insert(u);
            deg[u] += 1;
            deg[v] += 1;
        }
        let odds: Vec<usize> = (1..=n as usize).filter(|&i| deg[i] & 1 == 1).collect();
        match odds.len() {
            0 => true,
            2 => {
                let a = odds[0];
                let b = odds[1];
                if !adj[a].contains(&b) { return true; }
                for i in 1..=n as usize {
                    if i != a && i != b && !adj[a].contains(&i) && !adj[b].contains(&i) { return true; }
                }
                false
            }
            4 => {
                let a = odds[0]; let b = odds[1]; let c = odds[2]; let d = odds[3];
                (!adj[a].contains(&b) && !adj[c].contains(&d)) ||
                (!adj[a].contains(&c) && !adj[b].contains(&d)) ||
                (!adj[a].contains(&d) && !adj[b].contains(&c))
            }
            _ => false
        }
    }
}
```