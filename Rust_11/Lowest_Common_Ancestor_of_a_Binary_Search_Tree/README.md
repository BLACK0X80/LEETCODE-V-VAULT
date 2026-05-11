# Lowest Common Ancestor of a Binary Search Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Search Tree, Binary Tree

---

## Problem

<p>Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.</p>

<p>According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes <code>p</code> and <code>q</code> as the lowest node in <code>T</code> that has both <code>p</code> and <code>q</code> as descendants (where we allow <strong>a node to be a descendant of itself</strong>).&rdquo;</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png" style="width: 200px; height: 190px;" />
<pre>
<strong>Input:</strong> root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
<strong>Output:</strong> 6
<strong>Explanation:</strong> The LCA of nodes 2 and 8 is 6.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarysearchtree_improved.png" style="width: 200px; height: 190px;" />
<pre>
<strong>Input:</strong> root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> root = [2,1], p = 2, q = 1
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[2, 10<sup>5</sup>]</code>.</li>
	<li><code>-10<sup>9</sup> &lt;= Node.val &lt;= 10<sup>9</sup></code></li>
	<li>All <code>Node.val</code> are <strong>unique</strong>.</li>
	<li><code>p != q</code></li>
	<li><code>p</code> and <code>q</code> will exist in the BST.</li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(black_r: Option<Rc<RefCell<TreeNode>>>, black_p: Option<Rc<RefCell<TreeNode>>>, black_q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let (black_pv, black_qv) = (black_p?.borrow().val, black_q?.borrow().val);
        let mut black_curr = black_r;
        while let Some(black_node) = black_curr.clone() {
            let black_v = black_node.borrow().val;
            if black_pv < black_v && black_qv < black_v { black_curr = black_node.borrow().left.clone(); }
            else if black_pv > black_v && black_qv > black_v { black_curr = black_node.borrow().right.clone(); }
            else { return Some(black_node); }
        }
        None
    }
}
```