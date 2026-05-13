# Checking Existence of Edge Length Limited Paths

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Union-Find, Graph Theory, Sorting

---

## Problem

<p>An undirected graph of <code>n</code> nodes is defined by <code>edgeList</code>, where <code>edgeList[i] = [u<sub>i</sub>, v<sub>i</sub>, dis<sub>i</sub>]</code> denotes an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> with distance <code>dis<sub>i</sub></code>. Note that there may be <strong>multiple</strong> edges between two nodes.</p>

<p>Given an array <code>queries</code>, where <code>queries[j] = [p<sub>j</sub>, q<sub>j</sub>, limit<sub>j</sub>]</code>, your task is to determine for each <code>queries[j]</code> whether there is a path between <code>p<sub>j</sub></code> and <code>q<sub>j</sub></code><sub> </sub>such that each edge on the path has a distance <strong>strictly less than</strong> <code>limit<sub>j</sub></code> .</p>

<p>Return <em>a <strong>boolean array</strong> </em><code>answer</code><em>, where </em><code>answer.length == queries.length</code> <em>and the </em><code>j<sup>th</sup></code> <em>value of </em><code>answer</code> <em>is </em><code>true</code><em> if there is a path for </em><code>queries[j]</code><em> is </em><code>true</code><em>, and </em><code>false</code><em> otherwise</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/08/h.png" style="width: 267px; height: 262px;" />
<pre>
<strong>Input:</strong> n = 3, edgeList = [[0,1,2],[1,2,4],[2,0,8],[1,0,16]], queries = [[0,1,2],[0,2,5]]
<strong>Output:</strong> [false,true]
<strong>Explanation:</strong> The above figure shows the given graph. Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
For the first query, between 0 and 1 there is no path where each distance is less than 2, thus we return false for this query.
For the second query, there is a path (0 -&gt; 1 -&gt; 2) of two edges with distances less than 5, thus we return true for this query.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/08/q.png" style="width: 390px; height: 358px;" />
<pre>
<strong>Input:</strong> n = 5, edgeList = [[0,1,10],[1,2,5],[2,3,9],[3,4,13]], queries = [[0,4,14],[1,4,13]]
<strong>Output:</strong> [true,false]
<strong>Explanation:</strong> The above figure shows the given graph.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= edgeList.length, queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>edgeList[i].length == 3</code></li>
	<li><code>queries[j].length == 3</code></li>
	<li><code>0 &lt;= u<sub>i</sub>, v<sub>i</sub>, p<sub>j</sub>, q<sub>j</sub> &lt;= n - 1</code></li>
	<li><code>u<sub>i</sub> != v<sub>i</sub></code></li>
	<li><code>p<sub>j</sub> != q<sub>j</sub></code></li>
	<li><code>1 &lt;= dis<sub>i</sub>, limit<sub>j</sub> &lt;= 10<sup>9</sup></code></li>
	<li>There may be <strong>multiple</strong> edges between two nodes.</li>
</ul>


## Hints

1. All the queries are given in advance. Is there a way you can reorder the queries to avoid repeated computations?

## Solution

```rust
impl Solution {
    pub fn distance_limited_paths_exist(n: i32, mut edge_list: Vec<Vec<i32>>, mut queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut black_dsu: Vec<usize> = (0..n).collect();
        fn black_find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x { p[x] = black_find(p, p[x]); }
            p[x]
        }
        edge_list.sort_unstable_by_key(|e| e[2]);
        let q = queries.len();
        let mut black_idx: Vec<usize> = (0..q).collect();
        black_idx.sort_unstable_by_key(|&i| queries[i][2]);
        let mut black_ans = vec![false; q];
        let mut black_ei = 0;
        for &qi in &black_idx {
            let lim = queries[qi][2];
            while black_ei < edge_list.len() && edge_list[black_ei][2] < lim {
                let (u, v) = (edge_list[black_ei][0] as usize, edge_list[black_ei][1] as usize);
                let (pu, pv) = (black_find(&mut black_dsu, u), black_find(&mut black_dsu, v));
                if pu != pv { black_dsu[pu] = pv; }
                black_ei += 1;
            }
            let (p, q2) = (queries[qi][0] as usize, queries[qi][1] as usize);
            black_ans[qi] = black_find(&mut black_dsu, p) == black_find(&mut black_dsu, q2);
        }
        black_ans
    }
}
```