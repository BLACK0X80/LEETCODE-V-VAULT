# Remove Max Number of Edges to Keep Graph Fully Traversable

**Difficulty:** Hard
**Tags:** Union-Find, Graph Theory

---

## Problem

<p>Alice and Bob have an undirected graph of <code>n</code> nodes and three types of edges:</p>

<ul>
	<li>Type 1: Can be traversed by Alice only.</li>
	<li>Type 2: Can be traversed by Bob only.</li>
	<li>Type 3: Can be traversed by both Alice and Bob.</li>
</ul>

<p>Given an array <code>edges</code> where <code>edges[i] = [type<sub>i</sub>, u<sub>i</sub>, v<sub>i</sub>]</code> represents a bidirectional edge of type <code>type<sub>i</sub></code> between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>, find the maximum number of edges you can remove so that after removing the edges, the graph can still be fully traversed by both Alice and Bob. The graph is fully traversed by Alice and Bob if starting from any node, they can reach all other nodes.</p>

<p>Return <em>the maximum number of edges you can remove, or return</em> <code>-1</code> <em>if Alice and Bob cannot fully traverse the graph.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex1.png" style="width: 179px; height: 191px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
<strong>Output:</strong> 2
<strong>Explanation: </strong>If we remove the 2 edges [1,1,2] and [1,1,3]. The graph will still be fully traversable by Alice and Bob. Removing any additional edge will not make it so. So the maximum number of edges we can remove is 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex2.png" style="width: 178px; height: 190px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
<strong>Output:</strong> 0
<strong>Explanation: </strong>Notice that removing any edge will not make the graph fully traversable by Alice and Bob.
</pre>

<p><strong class="example">Example 3:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/08/19/ex3.png" style="width: 178px; height: 190px;" /></strong></p>

<pre>
<strong>Input:</strong> n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
<strong>Output:</strong> -1
<b>Explanation: </b>In the current graph, Alice cannot reach node 4 from the other nodes. Likewise, Bob cannot reach 1. Therefore it&#39;s impossible to make the graph fully traversable.</pre>

<p>&nbsp;</p>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= edges.length &lt;= min(10<sup>5</sup>, 3 * n * (n - 1) / 2)</code></li>
	<li><code>edges[i].length == 3</code></li>
	<li><code>1 &lt;= type<sub>i</sub> &lt;= 3</code></li>
	<li><code>1 &lt;= u<sub>i</sub> &lt; v<sub>i</sub> &lt;= n</code></li>
	<li>All tuples <code>(type<sub>i</sub>, u<sub>i</sub>, v<sub>i</sub>)</code> are distinct.</li>
</ul>


## Hints

1. Build the network instead of removing extra edges.
2. Suppose you have the final graph (after removing extra edges). Consider the subgraph with only the edges that Alice can traverse. What structure does this subgraph have? How many edges are there?
3. Use disjoint set union data structure for both Alice and Bob.
4. Always use Type 3 edges first, and connect the still isolated ones using other edges.

## Solution

```rust
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut black_a: Vec<usize> = (0..=n).collect();
        let mut black_b: Vec<usize> = (0..=n).collect();
        let mut black_ca = n - 1;
        let mut black_cb = n - 1;
        let mut black_rem = 0;

        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }
        fn black_union(p: &mut Vec<usize>, x: usize, y: usize) -> bool {
            let (px, py) = (black_find(p, x), black_find(p, y));
            if px == py { return false; }
            p[px] = py; true
        }

        for e in &edges {
            if e[0] == 3 {
                let ua = black_union(&mut black_a, e[1] as usize, e[2] as usize);
                let ub = black_union(&mut black_b, e[1] as usize, e[2] as usize);
                if ua { black_ca -= 1; } else if !ub { black_rem += 1; continue; }
                if ub { black_cb -= 1; }
            }
        }
        for e in &edges {
            if e[0] == 1 { if !black_union(&mut black_a, e[1] as usize, e[2] as usize) { black_rem += 1; } else { black_ca -= 1; } }
            if e[0] == 2 { if !black_union(&mut black_b, e[1] as usize, e[2] as usize) { black_rem += 1; } else { black_cb -= 1; } }
        }
        if black_ca != 0 || black_cb != 0 { -1 } else { black_rem }
    }
}
```