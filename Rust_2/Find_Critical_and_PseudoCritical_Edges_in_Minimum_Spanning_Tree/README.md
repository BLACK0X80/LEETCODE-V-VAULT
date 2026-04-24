# Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree

**Difficulty:** Hard
**Tags:** Union-Find, Graph Theory, Sorting, Minimum Spanning Tree, Strongly Connected Component

---

## Problem

<p>Given a weighted undirected connected graph with <code>n</code>&nbsp;vertices numbered from <code>0</code> to <code>n - 1</code>,&nbsp;and an array <code>edges</code>&nbsp;where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>, weight<sub>i</sub>]</code> represents a bidirectional and weighted edge between nodes&nbsp;<code>a<sub>i</sub></code>&nbsp;and <code>b<sub>i</sub></code>. A minimum spanning tree (MST) is a subset of the graph&#39;s edges that connects all vertices without cycles&nbsp;and with the minimum possible total edge weight.</p>

<p>Find <em>all the critical and pseudo-critical edges in the given graph&#39;s minimum spanning tree (MST)</em>. An MST edge whose deletion from the graph would cause the MST weight to increase is called a&nbsp;<em>critical edge</em>. On&nbsp;the other hand, a pseudo-critical edge is that which can appear in some MSTs but not all.</p>

<p>Note that you can return the indices of the edges in any order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/ex1.png" style="width: 259px; height: 262px;" /></p>

<pre>
<strong>Input:</strong> n = 5, edges = [[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]]
<strong>Output:</strong> [[0,1],[2,3,4,5]]
<strong>Explanation:</strong> The figure above describes the graph.
The following figure shows all the possible MSTs:
<img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/msts.png" style="width: 540px; height: 553px;" />
Notice that the two edges 0 and 1 appear in all MSTs, therefore they are critical edges, so we return them in the first list of the output.
The edges 2, 3, 4, and 5 are only part of some MSTs, therefore they are considered pseudo-critical edges. We add them to the second list of the output.
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/06/04/ex2.png" style="width: 247px; height: 253px;" /></p>

<pre>
<strong>Input:</strong> n = 4, edges = [[0,1,1],[1,2,1],[2,3,1],[0,3,1]]
<strong>Output:</strong> [[],[0,1,2,3]]
<strong>Explanation:</strong> We can observe that since all 4 edges have equal weight, choosing any 3 edges from the given 4 will yield an MST. Therefore all 4 edges are pseudo-critical.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 100</code></li>
	<li><code>1 &lt;= edges.length &lt;= min(200, n * (n - 1) / 2)</code></li>
	<li><code>edges[i].length == 3</code></li>
	<li><code>0 &lt;= a<sub>i</sub> &lt; b<sub>i</sub> &lt; n</code></li>
	<li><code>1 &lt;= weight<sub>i</sub>&nbsp;&lt;= 1000</code></li>
	<li>All pairs <code>(a<sub>i</sub>, b<sub>i</sub>)</code> are <strong>distinct</strong>.</li>
</ul>


## Hints

1. Use the Kruskal algorithm to find the minimum spanning tree by sorting the edges and picking edges from ones with smaller weights.
2. Use a disjoint set to avoid adding redundant edges that result in a cycle.
3. To find if one edge is critical, delete that edge and re-run the MST algorithm and see if the weight of the new MST increases.
4. To find if one edge is non-critical (in any MST), include that edge to the accepted edge list and continue the MST algorithm, then see if the resulting MST has the same weight of the initial MST of the entire graph.

## Solution

```rust
impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut indexed: Vec<(i32, i32, i32, usize)> = edges.iter().enumerate()
            .map(|(i, e)| (e[2], e[0], e[1], i)).collect();
        indexed.sort();

        fn find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = find(p, p[x]); }
            p[x]
        }

        let mst = |block: i32, pre: i32| -> i32 {
            let mut p: Vec<usize> = (0..n).collect();
            let mut w = 0i32;
            let mut cnt = 0;
            if pre >= 0 {
                let e = &indexed[pre as usize];
                let (pu, pv) = (find(&mut p, e.1 as usize), find(&mut p, e.2 as usize));
                p[pu] = pv; w += e.0; cnt += 1;
            }
            for (i, &(wt, u, v, _)) in indexed.iter().enumerate() {
                if i as i32 == block { continue; }
                let (pu, pv) = (find(&mut p, u as usize), find(&mut p, v as usize));
                if pu != pv { p[pu] = pv; w += wt; cnt += 1; }
            }
            if cnt < n - 1 { i32::MAX } else { w }
        };

        let base = mst(-1, -1);
        let mut critical = vec![];
        let mut pseudo = vec![];

        for i in 0..indexed.len() {
            if mst(i as i32, -1) > base { critical.push(indexed[i].3 as i32); }
            else if mst(-1, i as i32) == base { pseudo.push(indexed[i].3 as i32); }
        }
        vec![critical, pseudo]
    }
}
```