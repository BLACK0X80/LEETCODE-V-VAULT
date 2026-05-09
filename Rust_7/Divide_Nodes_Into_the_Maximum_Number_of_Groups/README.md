# Divide Nodes Into the Maximum Number of Groups

**Difficulty:** Hard
**Tags:** Depth-First Search, Breadth-First Search, Union-Find, Graph Theory

---

## Problem

<p>You are given a positive integer <code>n</code> representing the number of nodes in an <strong>undirected</strong> graph. The nodes are labeled from <code>1</code> to <code>n</code>.</p>

<p>You are also given a 2D integer array <code>edges</code>, where <code>edges[i] = [a<sub>i, </sub>b<sub>i</sub>]</code> indicates that there is a <strong>bidirectional</strong> edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. <strong>Notice</strong> that the given graph may be disconnected.</p>

<p>Divide the nodes of the graph into <code>m</code> groups (<strong>1-indexed</strong>) such that:</p>

<ul>
	<li>Each node in the graph belongs to exactly one group.</li>
	<li>For every pair of nodes in the graph that are connected by an edge <code>[a<sub>i, </sub>b<sub>i</sub>]</code>, if <code>a<sub>i</sub></code> belongs to the group with index <code>x</code>, and <code>b<sub>i</sub></code> belongs to the group with index <code>y</code>, then <code>|y - x| = 1</code>.</li>
</ul>

<p>Return <em>the maximum number of groups (i.e., maximum </em><code>m</code><em>) into which you can divide the nodes</em>. Return <code>-1</code> <em>if it is impossible to group the nodes with the given conditions</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/10/13/example1.png" style="width: 352px; height: 201px;" />
<pre>
<strong>Input:</strong> n = 6, edges = [[1,2],[1,4],[1,5],[2,6],[2,3],[4,6]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> As shown in the image we:
- Add node 5 to the first group.
- Add node 1 to the second group.
- Add nodes 2 and 4 to the third group.
- Add nodes 3 and 6 to the fourth group.
We can see that every edge is satisfied.
It can be shown that that if we create a fifth group and move any node from the third or fourth group to it, at least on of the edges will not be satisfied.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3, edges = [[1,2],[2,3],[3,1]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> If we add node 1 to the first group, node 2 to the second group, and node 3 to the third group to satisfy the first two edges, we can see that the third edge will not be satisfied.
It can be shown that no grouping is possible.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 500</code></li>
	<li><code>1 &lt;= edges.length &lt;= 10<sup>4</sup></code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>1 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt;= n</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li>There is at most one edge between any pair of vertices.</li>
</ul>


## Hints

1. If the graph is not bipartite, it is not possible to group the nodes.
2. Notice that we can solve the problem for each connected component independently, and the final answer will be just the sum of the maximum number of groups in each component.
3. Finally, to solve the problem for each connected component, we can notice that if for some node v we fix its position to be in the leftmost group, then we can also evaluate the position of every other node. That position is the depth of the node in a bfs tree after rooting at node v.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn magnificent_sets(black_n: i32, black_edges: Vec<Vec<i32>>) -> i32 {
        let black_n = black_n as usize;
        let mut black_adj = vec![vec![]; black_n + 1];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let mut black_vis = vec![false; black_n + 1];
        let mut black_res = 0;

        for black_i in 1..=black_n {
            if !black_vis[black_i] {
                let mut black_comp = vec![];
                let mut black_q = VecDeque::from([black_i]);
                black_vis[black_i] = true;
                while let Some(black_u) = black_q.pop_front() {
                    black_comp.push(black_u);
                    for &black_v in &black_adj[black_u] {
                        if !black_vis[black_v] {
                            black_vis[black_v] = true;
                            black_q.push_back(black_v);
                        }
                    }
                }

                let mut black_max = -1;
                for &black_root in &black_comp {
                    let mut black_d = vec![-1i32; black_n + 1];
                    let mut black_bq = VecDeque::from([black_root]);
                    black_d[black_root] = 1;
                    let mut black_cur = 1;
                    while let Some(black_u) = black_bq.pop_front() {
                        for &black_v in &black_adj[black_u] {
                            if black_d[black_v] == -1 {
                                black_d[black_v] = black_d[black_u] + 1;
                                black_cur = black_cur.max(black_d[black_v]);
                                black_bq.push_back(black_v);
                            } else if (black_d[black_v] - black_d[black_u]).abs() != 1 {
                                return -1;
                            }
                        }
                    }
                    black_max = black_max.max(black_cur);
                }
                black_res += black_max;
            }
        }
        black_res
    }
}
```