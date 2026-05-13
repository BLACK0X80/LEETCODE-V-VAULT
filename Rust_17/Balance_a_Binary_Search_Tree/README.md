# Balance a Binary Search Tree

**Difficulty:** Medium
**Tags:** Divide and Conquer, Greedy, Tree, Depth-First Search, Binary Search Tree, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary search tree, return <em>a <strong>balanced</strong> binary search tree with the same node values</em>. If there is more than one answer, return <strong>any of them</strong>.</p>

<p>A binary search tree is <strong>balanced</strong> if the depth of the two subtrees of every node never differs by more than <code>1</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/balance1-tree.jpg" style="width: 500px; height: 319px;" />
<pre>
<strong>Input:</strong> root = [1,null,2,null,3,null,4,null,null]
<strong>Output:</strong> [2,1,3,null,null,null,4]
<b>Explanation:</b> This is not the only correct answer, [3,1,4,null,2] is also correct.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/balanced2-tree.jpg" style="width: 224px; height: 145px;" />
<pre>
<strong>Input:</strong> root = [2,1,3]
<strong>Output:</strong> [2,1,3]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Convert the tree to a sorted array using an in-order traversal.
2. Construct a new balanced tree from the sorted array recursively.

## Solution

```rust
use std::{rc::Rc, cell::RefCell};impl Solution { pub fn balance_bst(black_root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> { let mut black_vals = vec![]; fn black_inorder(n: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) { if let Some(black_node) = n { black_inorder(black_node.borrow().left.clone(), v); v.push(black_node.borrow().val); black_inorder(black_node.borrow().right.clone(), v); } } fn black_build(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> { if v.is_empty() { return None; } let black_mid = v.len() / 2; Some(Rc::new(RefCell::new(TreeNode { val: v[black_mid], left: black_build(&v[..black_mid]), right: black_build(&v[black_mid + 1..]) }))) } black_inorder(black_root, &mut black_vals); black_build(&black_vals) } }
```