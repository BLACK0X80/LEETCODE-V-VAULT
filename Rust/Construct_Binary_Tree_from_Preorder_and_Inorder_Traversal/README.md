# Construct Binary Tree from Preorder and Inorder Traversal

**Difficulty:** Medium
**Tags:** Array, Hash Table, Divide and Conquer, Tree, Binary Tree

---

## Problem

<p>Given two integer arrays <code>preorder</code> and <code>inorder</code> where <code>preorder</code> is the preorder traversal of a binary tree and <code>inorder</code> is the inorder traversal of the same tree, construct and return <em>the binary tree</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
<pre>
<strong>Input:</strong> preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
<strong>Output:</strong> [3,9,20,null,null,15,7]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> preorder = [-1], inorder = [-1]
<strong>Output:</strong> [-1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= preorder.length &lt;= 3000</code></li>
	<li><code>inorder.length == preorder.length</code></li>
	<li><code>-3000 &lt;= preorder[i], inorder[i] &lt;= 3000</code></li>
	<li><code>preorder</code> and <code>inorder</code> consist of <strong>unique</strong> values.</li>
	<li>Each value of <code>inorder</code> also appears in <code>preorder</code>.</li>
	<li><code>preorder</code> is <strong>guaranteed</strong> to be the preorder traversal of the tree.</li>
	<li><code>inorder</code> is <strong>guaranteed</strong> to be the inorder traversal of the tree.</li>
</ul>



## Solution

```rust
use std::{rc::Rc, cell::RefCell, collections::HashMap};
impl Solution { pub fn build_tree(black_p: Vec<i32>, black_i: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> { let black_map: HashMap<i32, usize> = black_i.iter().enumerate().map(|(black_idx, &black_v)| (black_v, black_idx)).collect(); fn black_b(black_p: &[i32], black_p_s: usize, black_p_e: usize, black_i_s: usize, black_map: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> { if black_p_s > black_p_e { None } else { let black_v = black_p[black_p_s]; let black_m = black_map[&black_v]; let black_l_s = black_m - black_i_s; Some(Rc::new(RefCell::new(TreeNode { val: black_v, left: black_b(black_p, black_p_s + 1, black_p_s + black_l_s, black_i_s, black_map), right: black_b(black_p, black_p_s + black_l_s + 1, black_p_e, black_m + 1, black_map) }))) } } let black_n = black_p.len(); if black_n == 0 { None } else { black_b(&black_p, 0, black_n - 1, 0, &black_map) } } }
```