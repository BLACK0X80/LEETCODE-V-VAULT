# Minimum Weighted Subgraph With the Required Paths

**Difficulty:** Hard
**Tags:** Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>You are given an integer <code>n</code> denoting the number of nodes of a <strong>weighted directed</strong> graph. The nodes are numbered from <code>0</code> to <code>n - 1</code>.</p>

<p>You are also given a 2D integer array <code>edges</code> where <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>, weight<sub>i</sub>]</code> denotes that there exists a <strong>directed</strong> edge from <code>from<sub>i</sub></code> to <code>to<sub>i</sub></code> with weight <code>weight<sub>i</sub></code>.</p>

<p>Lastly, you are given three <strong>distinct</strong> integers <code>src1</code>, <code>src2</code>, and <code>dest</code> denoting three distinct nodes of the graph.</p>

<p>Return <em>the <strong>minimum weight</strong> of a subgraph of the graph such that it is <strong>possible</strong> to reach</em> <code>dest</code> <em>from both</em> <code>src1</code> <em>and</em> <code>src2</code> <em>via a set of edges of this subgraph</em>. In case such a subgraph does not exist, return <code>-1</code>.</p>

<p>A <strong>subgraph</strong> is a graph whose vertices and edges are subsets of the original graph. The <strong>weight</strong> of a subgraph is the sum of weights of its constituent edges.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/02/17/example1drawio.png" style="width: 263px; height: 250px;" />
<pre>
<strong>Input:</strong> n = 6, edges = [[0,2,2],[0,5,6],[1,0,3],[1,4,5],[2,1,1],[2,3,3],[2,3,4],[3,4,2],[4,5,1]], src1 = 0, src2 = 1, dest = 5
<strong>Output:</strong> 9
<strong>Explanation:</strong>
The above figure represents the input graph.
The blue edges represent one of the subgraphs that yield the optimal answer.
Note that the subgraph [[1,0,3],[0,5,6]] also yields the optimal answer. It is not possible to get a subgraph with less weight satisfying all the constraints.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/02/17/example2-1drawio.png" style="width: 350px; height: 51px;" />
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1,1],[2,1,1]], src1 = 0, src2 = 1, dest = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong>
The above figure represents the input graph.
It can be seen that there does not exist any path from node 1 to node 2, hence there are no subgraphs satisfying all the constraints.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= edges.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edges[i].length == 3</code></li>
	<li><code>0 &lt;= from<sub>i</sub>, to<sub>i</sub>, src1, src2, dest &lt;= n - 1</code></li>
	<li><code>from<sub>i</sub> != to<sub>i</sub></code></li>
	<li><code>src1</code>, <code>src2</code>, and <code>dest</code> are pairwise distinct.</li>
	<li><code>1 &lt;= weight[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Consider what the paths from src1 to dest and src2 to dest would look like in the optimal solution.
2. It can be shown that in an optimal solution, the two paths from src1 and src2 will coincide at one node, and the remaining part to dest will be the same for both paths. Now consider how to find the node where the paths will coincide.
3. How can algorithms for finding the shortest path between two nodes help us?

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        let mut rg = vec![vec![]; n];
        for e in &edges {
            g[e[0] as usize].push((e[1] as usize, e[2] as i64));
            rg[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }
        let dijkstra = |src: usize, adj: &Vec<Vec<(usize, i64)>>| {
            let mut dist = vec![i64::MAX; n];
            dist[src] = 0;
            let mut heap = BinaryHeap::new();
            heap.push(Reverse((0i64, src)));
            while let Some(Reverse((d, u))) = heap.pop() {
                if d > dist[u] { continue; }
                for &(v, w) in &adj[u] {
                    if dist[u] + w < dist[v] { dist[v] = dist[u] + w; heap.push(Reverse((dist[v], v))); }
                }
            }
            dist
        };
        let d1 = dijkstra(src1 as usize, &g);
        let d2 = dijkstra(src2 as usize, &g);
        let dd = dijkstra(dest as usize, &rg);
        let mut ans = i64::MAX;
        for i in 0..n {
            if d1[i] != i64::MAX && d2[i] != i64::MAX && dd[i] != i64::MAX {
                ans = ans.min(d1[i] + d2[i] + dd[i]);
            }
        }
        if ans == i64::MAX { -1 } else { ans }
    }
}
```