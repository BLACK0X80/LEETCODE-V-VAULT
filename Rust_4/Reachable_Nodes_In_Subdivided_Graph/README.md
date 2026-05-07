# Reachable Nodes In Subdivided Graph

**Difficulty:** Hard
**Tags:** Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>You are given an undirected graph (the <strong>&quot;original graph&quot;</strong>) with <code>n</code> nodes labeled from <code>0</code> to <code>n - 1</code>. You decide to <strong>subdivide</strong> each edge in the graph into a chain of nodes, with the number of new nodes varying between each edge.</p>

<p>The graph is given as a 2D array of <code>edges</code> where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, cnt<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> in the original graph, and <code>cnt<sub>i</sub></code> is the total number of new nodes that you will <strong>subdivide</strong> the edge into. Note that <code>cnt<sub>i</sub> == 0</code> means you will not subdivide the edge.</p>

<p>To <strong>subdivide</strong> the edge <code>[u<sub>i</sub>, v<sub>i</sub>]</code>, replace it with <code>(cnt<sub>i</sub> + 1)</code> new edges and <code>cnt<sub>i</sub></code> new nodes. The new nodes are <code>x<sub>1</sub></code>, <code>x<sub>2</sub></code>, ..., <code>x<sub>cnt<sub>i</sub></sub></code>, and the new edges are <code>[u<sub>i</sub>, x<sub>1</sub>]</code>, <code>[x<sub>1</sub>, x<sub>2</sub>]</code>, <code>[x<sub>2</sub>, x<sub>3</sub>]</code>, ..., <code>[x<sub>cnt<sub>i</sub>-1</sub>, x<sub>cnt<sub>i</sub></sub>]</code>, <code>[x<sub>cnt<sub>i</sub></sub>, v<sub>i</sub>]</code>.</p>

<p>In this <strong>new graph</strong>, you want to know how many nodes are <strong>reachable</strong> from the node <code>0</code>, where a node is <strong>reachable</strong> if the distance is <code>maxMoves</code> or less.</p>

<p>Given the original graph and <code>maxMoves</code>, return <em>the number of nodes that are <strong>reachable</strong> from node </em><code>0</code><em> in the new graph</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/01/origfinal.png" style="width: 600px; height: 247px;" />
<pre>
<strong>Input:</strong> edges = [[0,1,10],[0,2,1],[1,2,2]], maxMoves = 6, n = 3
<strong>Output:</strong> 13
<strong>Explanation:</strong> The edge subdivisions are shown in the image above.
The nodes that are reachable are highlighted in yellow.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> edges = [[0,1,4],[1,2,6],[0,2,8],[1,3,1]], maxMoves = 10, n = 4
<strong>Output:</strong> 23
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> edges = [[1,2,4],[1,4,5],[1,3,1],[2,3,4],[3,4,5]], maxMoves = 17, n = 5
<strong>Output:</strong> 1
<strong>Explanation:</strong> Node 0 is disconnected from the rest of the graph, so only node 0 is reachable.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= edges.length &lt;= min(n * (n - 1) / 2, 10<sup>4</sup>)</code></li>
	<li><code>edges[i].length == 3</code></li>
	<li><code>0 &lt;= u<sub>i</sub> &lt; v<sub>i</sub> &lt; n</code></li>
	<li>There are <strong>no multiple edges</strong> in the graph.</li>
	<li><code>0 &lt;= cnt<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= maxMoves &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= n &lt;= 3000</code></li>
</ul>



## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n];
        for e in &edges {
            let (u, v, c) = (e[0] as usize, e[1] as usize, e[2] as usize);
            black_adj[u].push((v, c));
            black_adj[v].push((u, c));
        }

        let mut black_dist = vec![i32::MAX; black_n];
        black_dist[0] = 0;
        let mut black_pq = BinaryHeap::new();
        black_pq.push(Reverse((0i32, 0usize)));

        while let Some(Reverse((d, u))) = black_pq.pop() {
            if d > black_dist[u] { continue; }
            for &(v, c) in &black_adj[u] {
                let nd = d + c as i32 + 1;
                if nd < black_dist[v] {
                    black_dist[v] = nd;
                    black_pq.push(Reverse((nd, v)));
                }
            }
        }

        let mut black_ans = black_dist.iter().filter(|&&d| d <= max_moves).count() as i32;

        for e in &edges {
            let (u, v, c) = (e[0] as usize, e[1] as usize, e[2] as i32);
            let black_from_u = if black_dist[u] <= max_moves { (max_moves - black_dist[u]).min(c) } else { 0 };
            let black_from_v = if black_dist[v] <= max_moves { (max_moves - black_dist[v]).min(c) } else { 0 };
            black_ans += (black_from_u + black_from_v).min(c);
        }
        black_ans
    }
}
```