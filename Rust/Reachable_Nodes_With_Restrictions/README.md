# Reachable Nodes With Restrictions

**Difficulty:** Medium
**Tags:** Array, Hash Table, Tree, Depth-First Search, Breadth-First Search, Union-Find, Graph Theory

---

## Problem

<p>There is an undirected tree with <code>n</code> nodes labeled from <code>0</code> to <code>n - 1</code> and <code>n - 1</code> edges.</p>

<p>You are given a 2D integer array <code>edges</code> of length <code>n - 1</code> where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree. You are also given an integer array <code>restricted</code> which represents <strong>restricted</strong> nodes.</p>

<p>Return <em>the <strong>maximum</strong> number of nodes you can reach from node </em><code>0</code><em> without visiting a restricted node.</em></p>

<p>Note that node <code>0</code> will <strong>not</strong> be a restricted node.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/06/15/ex1drawio.png" style="width: 402px; height: 322px;" />
<pre>
<strong>Input:</strong> n = 7, edges = [[0,1],[1,2],[3,1],[4,0],[0,5],[5,6]], restricted = [4,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The diagram above shows the tree.
We have that [0,1,2,3] are the only nodes that can be reached from node 0 without visiting a restricted node.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/06/15/ex2drawio.png" style="width: 412px; height: 312px;" />
<pre>
<strong>Input:</strong> n = 7, edges = [[0,1],[0,2],[0,5],[0,4],[3,2],[6,5]], restricted = [4,2,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The diagram above shows the tree.
We have that [0,5,6] are the only nodes that can be reached from node 0 without visiting a restricted node.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>edges.length == n - 1</code></li>
	<li><code>edges[i].length == 2</code></li>
	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
	<li><code>a<sub>i</sub> != b<sub>i</sub></code></li>
	<li><code>edges</code> represents a valid tree.</li>
	<li><code>1 &lt;= restricted.length &lt; n</code></li>
	<li><code>1 &lt;= restricted[i] &lt; n</code></li>
	<li>All the values of <code>restricted</code> are <strong>unique</strong>.</li>
</ul>


## Hints

1. Can we find all the reachable nodes in a single traversal?
2. Traverse the graph from node 0 while avoiding the nodes in restricted and do not revisit nodes that have been visited.
3. Keep count of how many nodes are visited in total.

## Solution

```rust
impl Solution { pub fn reachable_nodes(black_n: i32, black_e: Vec<Vec<i32>>, black_r: Vec<i32>) -> i32 { let mut black_res = vec![false; black_n as usize]; for &node in &black_r { black_res[node as usize] = true; } let mut black_f: Vec<usize> = (0..black_n as usize).collect(); let mut black_size = vec![1; black_n as usize]; fn black_find(mut i: usize, f: &mut Vec<usize>) -> usize { while f[i] != i { f[i] = f[f[i]]; i = f[i]; } i } for e in black_e { if !black_res[e[0] as usize] && !black_res[e[1] as usize] { let (r1, r2) = (black_find(e[0] as usize, &mut black_f), black_find(e[1] as usize, &mut black_f)); if r1 != r2 { black_f[r1] = r2; black_size[r2] += black_size[r1]; } } } black_size[black_find(0, &mut black_f)] } }
```