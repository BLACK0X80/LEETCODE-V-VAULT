# Validate Binary Search Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Search Tree, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, <em>determine if it is a valid binary search tree (BST)</em>.</p>

<p>A <strong>valid BST</strong> is defined as follows:</p>

<ul>
	<li>The left <span data-keyword="subtree">subtree</span> of a node contains only nodes with keys&nbsp;<strong>strictly less than</strong> the node&#39;s key.</li>
	<li>The right subtree of a node contains only nodes with keys <strong>strictly greater than</strong> the node&#39;s key.</li>
	<li>Both the left and right subtrees must also be binary search trees.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/tree1.jpg" style="width: 302px; height: 182px;" />
<pre>
<strong>Input:</strong> root = [2,1,3]
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/tree2.jpg" style="width: 422px; height: 292px;" />
<pre>
<strong>Input:</strong> root = [5,1,4,null,null,3,6]
<strong>Output:</strong> false
<strong>Explanation:</strong> The root node&#39;s value is 5 but its right child&#39;s value is 4.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>-2<sup>31</sup> &lt;= Node.val &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(black_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn black_check(black_node: &Option<Rc<RefCell<TreeNode>>>, black_min: Option<i64>, black_max: Option<i64>) -> bool {
            if let Some(black_n) = black_node {
                let black_v = black_n.borrow().val as i64;
                if black_min.map_or(false, |m| black_v <= m) || black_max.map_or(false, |m| black_v >= m) { return false; }
                black_check(&black_n.borrow().left, black_min, Some(black_v)) && 
                black_check(&black_n.borrow().right, Some(black_v), black_max)
            } else { true }
        }
        black_check(&black_root, None, None)
    }
}
```