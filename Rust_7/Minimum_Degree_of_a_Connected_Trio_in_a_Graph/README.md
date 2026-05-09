# Minimum Degree of a Connected Trio in a Graph

**Difficulty:** Hard
**Tags:** Graph Theory, Enumeration

---

## Problem

<p>You are given an undirected graph. You are given an integer <code>n</code> which is the number of nodes in the graph and an array <code>edges</code>, where each <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> indicates that there is an undirected edge between <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>.</p>

<p>A <strong>connected trio</strong> is a set of <strong>three</strong> nodes where there is an edge between <b>every</b> pair of them.</p>

<p>The <strong>degree of a connected trio</strong> is the number of edges where one endpoint is in the trio, and the other is not.</p>

<p>Return <em>the <strong>minimum</strong> degree of a connected trio in the graph, or</em> <code>-1</code> <em>if the graph has no connected trios.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/26/trios1.png" style="width: 388px; height: 164px;" />
<pre>
<strong>Input:</strong> n = 6, edges = [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There is exactly one trio, which is [1,2,3]. The edges that form its degree are bolded in the figure above.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/26/trios2.png" style="width: 388px; height: 164px;" />
<pre>
<strong>Input:</strong> n = 7, edges = [[1,3],[4,1],[4,3],[2,5],[5,6],[6,7],[7,5],[2,6]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are exactly three trios:
1) [1,4,3] with degree 0.
2) [2,5,6] with degree 2.
3) [5,6,7] with degree 2.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 400</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>1 &lt;= edges.length &lt;= n * (n-1) / 2</code></li>
	<li><code>1 &lt;= u<sub>i</sub>, v<sub>i</sub> &lt;= n</code></li>
	<li><code>u<sub>i </sub>!= v<sub>i</sub></code></li>
	<li>There are no repeated edges.</li>
</ul>


## Hints

1. Consider a trio with nodes u, v, and w. The degree of the trio is just degree(u) + degree(v) + degree(w) - 6. The -6 comes from subtracting the edges u-v, u-w, and v-w, which are counted twice each in the vertex degree calculation.
2. To get the trios (u,v,w), you can iterate on u, then iterate on each w,v such that w and v are neighbors of u and are neighbors of each other.

## Solution

```rust
impl Solution {
    pub fn min_trio_degree(black1: i32, black2: Vec<Vec<i32>>) -> i32 {
        let black3 = black1 as usize + 1;
        let mut black4 = vec![vec![false; black3]; black3];
        let mut black5 = vec![0; black3];
        for black6 in black2 {
            black4[black6[0] as usize][black6[1] as usize] = true;
            black4[black6[1] as usize][black6[0] as usize] = true;
            black5[black6[0] as usize] += 1;
            black5[black6[1] as usize] += 1;
        }
        let mut black7 = i32::MAX;
        for black8 in 1..black3 {
            for black9 in black8 + 1..black3 {
                if !black4[black8][black9] { continue; }
                for black10 in black9 + 1..black3 {
                    if black4[black8][black10] && black4[black9][black10] {
                        black7 = black7.min(black5[black8] + black5[black9] + black5[black10] - 6);
                    }
                }
            }
        }
        if black7 == i32::MAX { -1 } else { black7 }
    }
}
```