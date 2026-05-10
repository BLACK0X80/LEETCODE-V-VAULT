# Find Minimum Diameter After Merging Two Trees

**Difficulty:** Hard
**Tags:** Tree, Depth-First Search, Breadth-First Search, Graph Theory

---

## Problem

<p>There exist two <strong>undirected </strong>trees with <code>n</code> and <code>m</code> nodes, numbered from <code>0</code> to <code>n - 1</code> and from <code>0</code> to <code>m - 1</code>, respectively. You are given two 2D integer arrays <code>edges1</code> and <code>edges2</code> of lengths <code>n - 1</code> and <code>m - 1</code>, respectively, where <code>edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the first tree and <code>edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> in the second tree.</p>

<p>You must connect one node from the first tree with another node from the second tree with an edge.</p>

<p>Return the <strong>minimum </strong>possible <strong>diameter </strong>of the resulting tree.</p>

<p>The <strong>diameter</strong> of a tree is the length of the <em>longest</em> path between any two nodes in the tree.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong><img alt="" src="https://assets.leetcode.com/uploads/2024/04/22/example11-transformed.png" /></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges1 = [[0,1],[0,2],[0,3]], edges2 = [[0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>We can obtain a tree of diameter 3 by connecting node 0 from the first tree with any node from the second tree.</p>
</div>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2024/04/22/example211.png" />
<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">edges1 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]], edges2 = [[0,1],[0,2],[0,3],[2,4],[2,5],[3,6],[2,7]]</span></p>

<p><strong>Output:</strong> <span class="example-io">5</span></p>

<p><strong>Explanation:</strong></p>

<p>We can obtain a tree of diameter 5 by connecting node 0 from the first tree with node 0 from the second tree.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n, m &lt;= 10<sup>5</sup></code></li>
	<li><code>edges1.length == n - 1</code></li>
	<li><code>edges2.length == m - 1</code></li>
	<li><code>edges1[i].length == edges2[i].length == 2</code></li>
	<li><code>edges1[i] = [a<sub>i</sub>, b<sub>i</sub>]</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>edges2[i] = [u<sub>i</sub>, v<sub>i</sub>]</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt; m</code></li>
	<li>The input is generated such that <code>edges1</code> and <code>edges2</code> represent valid trees.</li>
</ul>


## Hints

1. Suppose that we connected node <code>a</code> in tree1 with node <code>b</code> in tree2. The diameter length of the resulting tree will be the largest of the following 3 values: 
<ol>
<li>The diameter of tree 1.</li>
<li>The diameter of tree 2.</li>
<li>The length of the longest path that starts at node <code>a</code> and that is completely within Tree 1 + The length of the longest path that starts at node <code>b</code> and that is completely within Tree 2 + 1.</li>
</ol> 
The added one in the third value is due to the additional edge that we have added between trees 1 and 2.
2. Values 1 and 2 are constant regardless of our choice of <code>a</code> and <code>b</code>. Therefore, we need to pick <code>a</code> and <code>b</code> in such a way that minimizes value 3.
3. If we pick <code>a</code> and <code>b</code> optimally, they will be in the diameters of Tree 1 and Tree 2, respectively. Exactly which nodes of the diameter should we pick?
4. <code>a</code> is the center of the diameter of tree 1, and <code>b</code> is the center of the diameter of tree 2.

## Solution

```rust
impl Solution {
    pub fn minimum_diameter_after_merge(black_e1: Vec<Vec<i32>>, black_e2: Vec<Vec<i32>>) -> i32 {
        let black_d1 = Self::black_get_dist(&black_e1);
        let black_d2 = Self::black_get_dist(&black_e2);
        
        let black_res = black_d1.max(black_d2);
        let black_combined = (black_d1 + 1) / 2 + (black_d2 + 1) / 2 + 1;
        
        black_res.max(black_combined)
    }

    fn black_get_dist(black_edges: &Vec<Vec<i32>>) -> i32 {
        let black_n = black_edges.len() + 1;
        if black_n <= 1 { return 0; }
        
        let mut black_adj = vec![vec![]; black_n];
        for black_e in black_edges {
            black_adj[black_e[0] as usize].push(black_e[1] as usize);
            black_adj[black_e[1] as usize].push(black_e[0] as usize);
        }

        let (black_node_a, _) = Self::black_bfs(0, &black_adj);
        let (_, black_diameter) = Self::black_bfs(black_node_a, &black_adj);
        
        black_diameter
    }

    fn black_bfs(black_start: usize, black_adj: &Vec<Vec<usize>>) -> (usize, i32) {
        let black_n = black_adj.len();
        let mut black_dist = vec![-1; black_n];
        let mut black_q = std::collections::VecDeque::from([black_start]);
        black_dist[black_start] = 0;
        
        let mut black_farthest_node = black_start;
        let mut black_max_d = 0;

        while let Some(black_u) = black_q.pop_front() {
            if black_dist[black_u] > black_max_d {
                black_max_d = black_dist[black_u];
                black_farthest_node = black_u;
            }
            for &black_v in &black_adj[black_u] {
                if black_dist[black_v] == -1 {
                    black_dist[black_v] = black_dist[black_u] + 1;
                    black_q.push_back(black_v);
                }
            }
        }
        (black_farthest_node, black_max_d)
    }
}
```