# Find Edges in Shortest Paths

**Difficulty:** Hard
**Tags:** Depth-First Search, Breadth-First Search, Graph Theory, Heap (Priority Queue), Shortest Path

---

## Problem

<p>You are given an undirected weighted graph of <code>n</code> nodes numbered from 0 to <code>n - 1</code>. The graph consists of <code>m</code> edges represented by a 2D array <code>edges</code>, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>, w<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> with weight <code>w<sub>i</sub></code>.</p>

<p>Consider all the shortest paths from node 0 to node <code>n - 1</code> in the graph. You need to find a <strong>boolean</strong> array <code>answer</code> where <code>answer[i]</code> is <code>true</code> if the edge <code>edges[i]</code> is part of <strong>at least</strong> one shortest path. Otherwise, <code>answer[i]</code> is <code>false</code>.</p>

<p>Return the array <code>answer</code>.</p>

<p><strong>Note</strong> that the graph may not be connected.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/03/05/graph35drawio-1.png" style="height: 129px; width: 250px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 6, edges = [[0,1,4],[0,2,1],[1,3,2],[1,4,3],[1,5,1],[2,3,1],[3,5,3],[4,5,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[true,true,true,false,true,true,true,false]</span></p>

<p><strong>Explanation:</strong></p>

<p>The following are <strong>all</strong> the shortest paths between nodes 0 and 5:</p>

<ul>
	<li>The path <code>0 -&gt; 1 -&gt; 5</code>: The sum of weights is <code>4 + 1 = 5</code>.</li>
	<li>The path <code>0 -&gt; 2 -&gt; 3 -&gt; 5</code>: The sum of weights is <code>1 + 1 + 3 = 5</code>.</li>
	<li>The path <code>0 -&gt; 2 -&gt; 3 -&gt; 1 -&gt; 5</code>: The sum of weights is <code>1 + 1 + 2 + 1 = 5</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/03/05/graphhhh.png" style="width: 185px; height: 136px;" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 4, edges = [[2,0,1],[0,1,1],[0,3,4],[3,2,2]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[true,false,false,true]</span></p>

<p><strong>Explanation:</strong></p>

<p>There is one shortest path between nodes 0 and 3, which is the path <code>0 -&gt; 2 -&gt; 3</code> with the sum of weights <code>1 + 2 = 3</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>m == edges.length</code></li>
	<li><code>1 &lt;= m &lt;= min(5 * 10<sup>4</sup>, n * (n - 1) / 2)</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li><code>1 &lt;= w<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
	<li>There are no repeated edges.</li>
</ul>


## Hints

1. Find all the shortest paths starting from nodes 0 and <code>n - 1</code> to all other nodes.
2. How to use the above calculated shortest paths to check if an edge is part of at least one shortest path from 0 to <code>n - 1</code>?

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Black {
    cost: i64,
    node: usize,
}
impl Ord for Black {
    fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) }
}
impl PartialOrd for Black {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut black_adj = vec![vec![]; n];
        for e in &edges {
            black_adj[e[0] as usize].push((e[1] as usize, e[2] as i64));
            black_adj[e[1] as usize].push((e[0] as usize, e[2] as i64));
        }

        fn black_dij(n: usize, start: usize, adj: &[Vec<(usize, i64)>]) -> Vec<i64> {
            let mut dist = vec![i64::MAX; n];
            dist[start] = 0;
            let mut pq = BinaryHeap::new();
            pq.push(Black { cost: 0, node: start });
            while let Some(Black { cost, node }) = pq.pop() {
                if cost > dist[node] { continue; }
                for &(v, w) in &adj[node] {
                    if cost + w < dist[v] {
                        dist[v] = cost + w;
                        pq.push(Black { cost: dist[v], node: v });
                    }
                }
            }
            dist
        }

        let black_d1 = black_dij(n, 0, &black_adj);
        let black_d2 = black_dij(n, n - 1, &black_adj);
        let black_short = black_d1[n - 1];
        if black_short == i64::MAX {
            return vec![false; edges.len()];
        }

        edges.iter().map(|e| {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
            if black_d1[u] != i64::MAX && black_d2[v] != i64::MAX && black_d1[u] + w + black_d2[v] == black_short { return true; }
            if black_d1[v] != i64::MAX && black_d2[u] != i64::MAX && black_d1[v] + w + black_d2[u] == black_short { return true; }
            false
        }).collect()
    }
}
```