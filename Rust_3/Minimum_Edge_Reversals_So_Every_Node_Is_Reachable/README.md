# Minimum Edge Reversals So Every Node Is Reachable

**Difficulty:** Hard
**Tags:** Dynamic Programming, Depth-First Search, Breadth-First Search, Graph Theory

---

## Problem

<p>There is a <strong>simple directed graph</strong> with <code>n</code> nodes labeled from <code>0</code> to <code>n - 1</code>. The graph would form a <strong>tree</strong> if its edges were bi-directional.</p>

<p>You are given an integer <code>n</code> and a <strong>2D</strong> integer array <code>edges</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> represents a <strong>directed edge</strong> going from node <code>u<sub>i</sub></code> to node <code>v<sub>i</sub></code>.</p>

<p>An <strong>edge reversal</strong> changes the direction of an edge, i.e., a directed edge going from node <code>u<sub>i</sub></code> to node <code>v<sub>i</sub></code> becomes a directed edge going from node <code>v<sub>i</sub></code> to node <code>u<sub>i</sub></code>.</p>

<p>For every node <code>i</code> in the range <code>[0, n - 1]</code>, your task is to <strong>independently</strong> calculate the <strong>minimum</strong> number of <strong>edge reversals</strong> required so it is possible to reach any other node starting from node <code>i</code> through a <strong>sequence</strong> of <strong>directed edges</strong>.</p>

<p>Return <em>an integer array </em><code>answer</code><em>, where </em><code>answer[i]</code><em> is the</em><em> </em> <em><strong>minimum</strong> number of <strong>edge reversals</strong> required so it is possible to reach any other node starting from node </em><code>i</code><em> through a <strong>sequence</strong> of <strong>directed edges</strong>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img height="246" src="https://assets.leetcode.com/uploads/2023/08/26/image-20230826221104-3.png" width="312" /></p>

<pre>
<strong>Input:</strong> n = 4, edges = [[2,0],[2,1],[1,3]]
<strong>Output:</strong> [1,1,0,2]
<strong>Explanation:</strong> The image above shows the graph formed by the edges.
For node 0: after reversing the edge [2,0], it is possible to reach any other node starting from node 0.
So, answer[0] = 1.
For node 1: after reversing the edge [2,1], it is possible to reach any other node starting from node 1.
So, answer[1] = 1.
For node 2: it is already possible to reach any other node starting from node 2.
So, answer[2] = 0.
For node 3: after reversing the edges [1,3] and [2,1], it is possible to reach any other node starting from node 3.
So, answer[3] = 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img height="217" src="https://assets.leetcode.com/uploads/2023/08/26/image-20230826225541-2.png" width="322" /></p>

<pre>
<strong>Input:</strong> n = 3, edges = [[1,2],[2,0]]
<strong>Output:</strong> [2,0,1]
<strong>Explanation:</strong> The image above shows the graph formed by the edges.
For node 0: after reversing the edges [2,0] and [1,2], it is possible to reach any other node starting from node 0.
So, answer[0] = 2.
For node 1: it is already possible to reach any other node starting from node 1.
So, answer[1] = 0.
For node 2: after reversing the edge [1, 2], it is possible to reach any other node starting from node 2.
So, answer[2] = 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>0 &lt;= u<sub>i</sub> == edges[i][0] &lt; n</code></li>
	<li><code>0 &lt;= v<sub>i</sub> == edges[i][1] &lt; n</code></li>
	<li><code>u<sub>i</sub> != v<sub>i</sub></code></li>
	<li>The input is generated such&nbsp;that if the edges were bi-directional, the graph would be a tree.</li>
</ul>


## Hints

1. The problem can be solved using tree DP.
2. Using node <code>0</code> as the root, let <code>dp[x]</code> be the minimum number of edge reversals so node <code>x</code> can reach every node in its subtree.
3. Using a DFS traversing the edges bidirectionally, we can compute <code>dp</code>.<br />
<code>dp[x] = dp[y] +</code> (<code>1</code> if the edge between <code>x</code> and <code>y</code> is going from <code>y</code> to <code>x</code>; <code>0</code> otherwise), where <code>x</code> is the parent of <code>y</code>.
4. Let <code>answer[x]</code> be the minimum number of edge reversals so it is possible to reach any other node starting from node <code>x</code>.
5. Using another DFS starting from node <code>0</code> and traversing the edges bidirectionally, we can compute <code>answer</code>.<br />
<code>answer[0] = dp[0]</code><br />
<code>answer[y] = answer[x] +</code> (<code>1</code> if the edge between <code>x</code> and <code>y</code> is going from <code>x</code> to <code>y</code>; <code>-1</code> otherwise), where <code>x</code> is the parent of <code>y</code>.

## Solution

```rust
impl Solution {
    pub fn min_edge_reversals(black_n: i32, black_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n_usize = black_n as usize;
        let mut black_adj = vec![vec![]; black_n_usize];
        
        for black_edge in black_edges {
            let (black_u, black_v) = (black_edge[0] as usize, black_edge[1] as usize);
            black_adj[black_u].push((black_v, 0)); 
            black_adj[black_v].push((black_u, 1)); 
        }

        let mut black_total_reversals = 0;
        Self::black_dfs_init(0, 0, &black_adj, &mut black_total_reversals);

        let mut black_ans = vec![0; black_n_usize];
        black_ans[0] = black_total_reversals;
        
        Self::black_dfs_reroot(0, 0, &black_adj, &mut black_ans);
        black_ans
    }

    fn black_dfs_init(black_u: usize, black_p: usize, black_adj: &Vec<Vec<(usize, i32)>>, black_sum: &mut i32) {
        for &(black_v, black_cost) in &black_adj[black_u] {
            if black_v != black_p {
                *black_sum += black_cost;
                Self::black_dfs_init(black_v, black_u, black_adj, black_sum);
            }
        }
    }

    fn black_dfs_reroot(black_u: usize, black_p: usize, black_adj: &Vec<Vec<(usize, i32)>>, black_ans: &mut Vec<i32>) {
        for &(black_v, black_cost) in &black_adj[black_u] {
            if black_v != black_p {
                let bravexuneth = if black_cost == 1 { -1 } else { 1 };
                black_ans[black_v] = black_ans[black_u] + bravexuneth;
                Self::black_dfs_reroot(black_v, black_u, black_adj, black_ans);
            }
        }
    }
}
```