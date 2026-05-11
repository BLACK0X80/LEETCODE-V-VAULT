# Construct Binary Tree from Preorder and Postorder Traversal

**Difficulty:** Medium
**Tags:** Array, Hash Table, Divide and Conquer, Tree, Binary Tree

---

## Problem

<p>Given two integer arrays, <code>preorder</code> and <code>postorder</code> where <code>preorder</code> is the preorder traversal of a binary tree of <strong>distinct</strong> values and <code>postorder</code> is the postorder traversal of the same tree, reconstruct and return <em>the binary tree</em>.</p>

<p>If there exist multiple answers, you can <strong>return any</strong> of them.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/07/24/lc-prepost.jpg" style="width: 304px; height: 265px;" />
<pre>
<strong>Input:</strong> preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
<strong>Output:</strong> [1,2,3,4,5,6,7]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> preorder = [1], postorder = [1]
<strong>Output:</strong> [1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= preorder.length &lt;= 30</code></li>
	<li><code>1 &lt;= preorder[i] &lt;= preorder.length</code></li>
	<li>All the values of <code>preorder</code> are <strong>unique</strong>.</li>
	<li><code>postorder.length == preorder.length</code></li>
	<li><code>1 &lt;= postorder[i] &lt;= postorder.length</code></li>
	<li>All the values of <code>postorder</code> are <strong>unique</strong>.</li>
	<li>It is guaranteed that <code>preorder</code> and <code>postorder</code> are the preorder traversal and postorder traversal of the same binary tree.</li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell;
impl Solution { pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> { Self::build(&pre, &post) } fn build(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> { if pre.is_empty() { return None; } let mut black_root = TreeNode::new(pre[0]); if pre.len() > 1 { let black_left_size = post.iter().position(|&x| x == pre[1]).unwrap() + 1; black_root.left = Self::build(&pre[1..black_left_size + 1], &post[..black_left_size]); black_root.right = Self::build(&pre[black_left_size + 1..], &post[black_left_size..post.len() - 1]); } Some(Rc::new(RefCell::new(black_root))) } }
```