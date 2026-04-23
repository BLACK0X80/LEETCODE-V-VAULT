# Construct Binary Tree from Inorder and Postorder Traversal

**Difficulty:** Medium
**Tags:** Array, Hash Table, Divide and Conquer, Tree, Binary Tree

---

## Problem

<p>Given two integer arrays <code>inorder</code> and <code>postorder</code> where <code>inorder</code> is the inorder traversal of a binary tree and <code>postorder</code> is the postorder traversal of the same tree, construct and return <em>the binary tree</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
<pre>
<strong>Input:</strong> inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
<strong>Output:</strong> [3,9,20,null,null,15,7]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> inorder = [-1], postorder = [-1]
<strong>Output:</strong> [-1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= inorder.length &lt;= 3000</code></li>
	<li><code>postorder.length == inorder.length</code></li>
	<li><code>-3000 &lt;= inorder[i], postorder[i] &lt;= 3000</code></li>
	<li><code>inorder</code> and <code>postorder</code> consist of <strong>unique</strong> values.</li>
	<li>Each value of <code>postorder</code> also appears in <code>inorder</code>.</li>
	<li><code>inorder</code> is <strong>guaranteed</strong> to be the inorder traversal of the tree.</li>
	<li><code>postorder</code> is <strong>guaranteed</strong> to be the postorder traversal of the tree.</li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (i, &v) in inorder.iter().enumerate() { map.insert(v, i); }
        Self::build(&inorder, &postorder, 0, inorder.len(), 0, postorder.len(), &map)
    }
    fn build(inorder: &[i32], postorder: &[i32], il: usize, ir: usize, pl: usize, pr: usize, map: &HashMap<i32, usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if il >= ir || pl >= pr { return None; }
        let root_val = postorder[pr - 1];
        let idx = map[&root_val];
        let left_len = idx - il;
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Self::build(inorder, postorder, il, idx, pl, pl + left_len, map),
            right: Self::build(inorder, postorder, idx + 1, ir, pl + left_len, pr - 1, map),
        })))
    }
}
```