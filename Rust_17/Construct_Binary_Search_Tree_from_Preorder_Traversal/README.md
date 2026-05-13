# Construct Binary Search Tree from Preorder Traversal

**Difficulty:** Medium
**Tags:** Array, Stack, Tree, Binary Search Tree, Monotonic Stack, Binary Tree

---

## Problem

<p>Given an array of integers preorder, which represents the <strong>preorder traversal</strong> of a BST (i.e., <strong>binary search tree</strong>), construct the tree and return <em>its root</em>.</p>

<p>It is <strong>guaranteed</strong> that there is always possible to find a binary search tree with the given requirements for the given test cases.</p>

<p>A <strong>binary search tree</strong> is a binary tree where for every node, any descendant of <code>Node.left</code> has a value <strong>strictly less than</strong> <code>Node.val</code>, and any descendant of <code>Node.right</code> has a value <strong>strictly greater than</strong> <code>Node.val</code>.</p>

<p>A <strong>preorder traversal</strong> of a binary tree displays the value of the node first, then traverses <code>Node.left</code>, then traverses <code>Node.right</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/03/06/1266.png" style="height: 386px; width: 590px;" />
<pre>
<strong>Input:</strong> preorder = [8,5,1,7,10,12]
<strong>Output:</strong> [8,5,10,1,7,null,12]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> preorder = [1,3]
<strong>Output:</strong> [1,null,3]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= preorder.length &lt;= 100</code></li>
	<li><code>1 &lt;= preorder[i] &lt;= 1000</code></li>
	<li>All the values of <code>preorder</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
use std::{rc::Rc, cell::RefCell};impl Solution { pub fn bst_from_preorder(black_p: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> { fn black_b(black_v: &[i32], black_m: i32) -> (Option<Rc<RefCell<TreeNode>>>, &[i32]) { if black_v.is_empty() || black_v[0] > black_m { return (None, black_v); } let mut black_node = TreeNode::new(black_v[0]); let (black_l, black_rem) = black_b(&black_v[1..], black_node.val); let (black_r, black_fin) = black_b(black_rem, black_m); black_node.left = black_l; black_node.right = black_r; (Some(Rc::new(RefCell::new(black_node))), black_fin) } black_b(&black_p, i32::MAX).0 } }
```