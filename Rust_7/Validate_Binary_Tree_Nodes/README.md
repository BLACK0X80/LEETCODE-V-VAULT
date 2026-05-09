# Validate Binary Tree Nodes

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Union-Find, Graph Theory, Binary Tree

---

## Problem

<p>You have <code>n</code> binary tree nodes numbered from <code>0</code> to <code>n - 1</code> where node <code>i</code> has two children <code>leftChild[i]</code> and <code>rightChild[i]</code>, return <code>true</code> if and only if <strong>all</strong> the given nodes form <strong>exactly one</strong> valid binary tree.</p>

<p>If node <code>i</code> has no left child then <code>leftChild[i]</code> will equal <code>-1</code>, similarly for the right child.</p>

<p>Note that the nodes have no values and that we only use the node numbers in this problem.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex1.png" style="width: 195px; height: 287px;" />
<pre>
<strong>Input:</strong> n = 4, leftChild = [1,-1,3,-1], rightChild = [2,-1,-1,-1]
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex2.png" style="width: 183px; height: 272px;" />
<pre>
<strong>Input:</strong> n = 4, leftChild = [1,-1,3,-1], rightChild = [2,3,-1,-1]
<strong>Output:</strong> false
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/08/23/1503_ex3.png" style="width: 82px; height: 174px;" />
<pre>
<strong>Input:</strong> n = 2, leftChild = [1,0], rightChild = [-1,-1]
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == leftChild.length == rightChild.length</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>-1 &lt;= leftChild[i], rightChild[i] &lt;= n - 1</code></li>
</ul>


## Hints

1. Find the parent of each node.
2. A valid tree must have nodes with only one parent and exactly one node with no parent.

## Solution

```rust
impl Solution { pub fn validate_binary_tree_nodes(black_n: i32, black_l: Vec<i32>, black_r: Vec<i32>) -> bool { let mut black_f: Vec<usize> = (0..black_n as usize).collect(); let mut black_cnt = black_n; fn black_find(mut i: usize, f: &mut Vec<usize>) -> usize { while f[i] != i { f[i] = f[f[i]]; i = f[i]; } i } let mut black_in = vec![0; black_n as usize]; for (i, (&left, &right)) in black_l.iter().zip(black_r.iter()).enumerate() { for &child in &[left, right] { if child != -1 { black_in[child as usize] += 1; if black_in[child as usize] > 1 { return false; } let (r1, r2) = (black_find(i, &mut black_f), black_find(child as usize, &mut black_f)); if r1 == r2 { return false; } black_f[r2] = r1; black_cnt -= 1; } } } black_cnt == 1 } }
```