# Count Subtrees With Max Distance Between Cities

**Difficulty:** Hard
**Tags:** Dynamic Programming, Bit Manipulation, Tree, Enumeration, Bitmask

---

## Problem

<p>There are <code>n</code> cities numbered from <code>1</code> to <code>n</code>. You are given an array <code>edges</code> of size <code>n-1</code>, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> represents a bidirectional edge between cities <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>. There exists a unique path between each pair of cities. In other words, the cities form a <strong>tree</strong>.</p>

<p>A <strong>subtree</strong> is a subset of cities where every city is reachable from every other city in the subset, where the path between each pair passes through only the cities from the subset. Two subtrees are different if there is a city in one subtree that is not present in the other.</p>

<p>For each <code>d</code> from <code>1</code> to <code>n-1</code>, find the number of subtrees in which the <strong>maximum distance</strong> between any two cities in the subtree is equal to <code>d</code>.</p>

<p>Return <em>an array of size</em> <code>n-1</code> <em>where the </em><code>d<sup>th</sup></code><em> </em><em>element <strong>(1-indexed)</strong> is the number of subtrees in which the <strong>maximum distance</strong> between any two cities is equal to </em><code>d</code>.</p>

<p><strong>Notice</strong>&nbsp;that&nbsp;the <strong>distance</strong> between the two cities is the number of edges in the path between them.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/p1.png" style="width: 161px; height: 181px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 4, edges = [[1,2],[2,3],[2,4]]
<strong>Output:</strong> [3,4,0]
<strong>Explanation:
</strong>The subtrees with subsets {1,2}, {2,3} and {2,4} have a max distance of 1.
The subtrees with subsets {1,2,3}, {1,2,4}, {2,3,4} and {1,2,3,4} have a max distance of 2.
No subtree has two nodes where the max distance between them is 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2, edges = [[1,2]]
<strong>Output:</strong> [1]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 3, edges = [[1,2],[2,3]]
<strong>Output:</strong> [2,1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 15</code></li>
	<li><code>edges.length == n-1</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>1 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n</code></li>
	<li>All pairs <code>(u<sub>i</sub>, v<sub>i</sub>)</code> are distinct.</li>
</ul>

## Hints

1. Iterate through every possible subtree by doing a bitmask on which vertices to include. How can you determine if a subtree is valid (all vertices are connected)?
2. To determine connectivity, count the number of reachable vertices starting from any included vertex and only traveling on edges connecting 2 vertices in the subtree. The count should be the same as the number of 1s in the bitmask.
3. The diameter is basically the maximum distance between any two nodes. Root the tree at a vertex. The answer is the max of the heights of the two largest subtrees or the longest diameter in any of the subtrees.

## Solution

```rust
impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = n as usize;
        let mut black_adj = vec![vec![]; black_n];
        let mut black_dist = vec![vec![100; black_n]; black_n];
        for e in &edges {
            let u = e[0] as usize - 1;
            let v = e[1] as usize - 1;
            black_adj[u].push(v); black_adj[v].push(u);
            black_dist[u][v] = 1; black_dist[v][u] = 1;
            black_dist[u][u] = 0; black_dist[v][v] = 0;
        }
        for k in 0..black_n {
            for i in 0..black_n {
                for j in 0..black_n {
                    black_dist[i][j] = black_dist[i][j].min(black_dist[i][k] + black_dist[k][j]);
                }
            }
        }
        let mut black_ans = vec![0; black_n - 1];
        for mask in 1..(1 << black_n) {
            let nodes: Vec<usize> = (0..black_n).filter(|i| (mask >> i) & 1 == 1).collect();
            if nodes.len() < 2 { continue; }
            let mut black_q = vec![nodes[0]];
            let mut black_vis = 1 << nodes[0];
            let mut head = 0;
            while head < black_q.len() {
                let u = black_q[head]; head += 1;
                for &v in &black_adj[u] {
                    if (mask >> v) & 1 == 1 && black_vis & (1 << v) == 0 {
                        black_vis |= 1 << v;
                        black_q.push(v);
                    }
                }
            }
            if black_vis != mask { continue; }
            let mut black_max = 0;
            for i in 0..nodes.len() {
                for j in i + 1..nodes.len() {
                    black_max = black_max.max(black_dist[nodes[i]][nodes[j]]);
                }
            }
            if black_max > 0 && black_max < black_n {
                black_ans[black_max - 1] += 1;
            }
        }
        black_ans
    }
}
```