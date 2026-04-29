# Path In Zigzag Labelled Binary Tree

**Difficulty:** Medium
**Tags:** Math, Tree, Binary Tree

---

## Problem

<p>In an infinite binary tree where every node has two children, the nodes are labelled in row order.</p>

<p>In the odd numbered rows (ie., the first, third, fifth,...), the labelling is left to right, while in the even numbered rows (second, fourth, sixth,...), the labelling is right to left.</p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2019/06/24/tree.png" style="width: 300px; height: 138px;" /></p>

<p>Given the <code>label</code> of a node in this tree, return the labels in the path from the root of the tree to the&nbsp;node with that <code>label</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> label = 14
<strong>Output:</strong> [1,3,4,14]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> label = 26
<strong>Output:</strong> [1,2,6,10,26]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= label &lt;= 10^6</code></li>
</ul>


## Hints

1. Based on the label of the current node, find what the label must be for the parent of that node.

## Solution

```rust
impl Solution { pub fn path_in_zig_zag_tree(mut black_l: i32) -> Vec<i32> { let mut black_res = vec![]; while black_l > 0 { black_res.push(black_l); let black_d = (black_l as f64).log2() as u32; black_l = ((1 << black_d) + (1 << (black_d + 1)) - 1 - black_l) / 2; } black_res.reverse(); black_res } }
```