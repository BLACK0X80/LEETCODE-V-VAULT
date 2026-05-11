# Recover Binary Search Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Search Tree, Binary Tree

---

## Problem

<p>You are given the <code>root</code> of a binary search tree (BST), where the values of <strong>exactly</strong> two nodes of the tree were swapped by mistake. <em>Recover the tree without changing its structure</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover1.jpg" style="width: 422px; height: 302px;" />
<pre>
<strong>Input:</strong> root = [1,3,null,null,2]
<strong>Output:</strong> [3,1,null,null,2]
<strong>Explanation:</strong> 3 cannot be a left child of 1 because 3 &gt; 1. Swapping 1 and 3 makes the BST valid.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover2.jpg" style="width: 581px; height: 302px;" />
<pre>
<strong>Input:</strong> root = [3,1,4,null,null,2]
<strong>Output:</strong> [2,1,4,null,null,3]
<strong>Explanation:</strong> 2 cannot be in the right subtree of 3 because 2 &lt; 3. Swapping 2 and 3 makes the BST valid.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[2, 1000]</code>.</li>
	<li><code>-2<sup>31</sup> &lt;= Node.val &lt;= 2<sup>31</sup> - 1</code></li>
</ul>

<p>&nbsp;</p>
<strong>Follow up:</strong> A solution using <code>O(n)</code> space is pretty straight-forward. Could you devise a constant <code>O(1)</code> space solution?


## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn recover_tree(black_root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut black_f, mut black_s): (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) = (None, None);
        let (mut black_p, mut black_curr) = (None::<Rc<RefCell<TreeNode>>>, black_root.clone());
        
        while let Some(black_node) = black_curr.clone() {
            if black_node.borrow().left.is_none() {
                if let Some(ref black_prev) = black_p {
                    if black_prev.borrow().val > black_node.borrow().val {
                        if black_f.is_none() { black_f = black_p.clone(); }
                        black_s = Some(black_node.clone());
                    }
                }
                black_p = Some(black_node.clone()); 
                black_curr = black_node.borrow().right.clone();
            } else {
                let mut black_pre = black_node.borrow().left.clone().unwrap();
                while black_pre.borrow().right.is_some() && black_pre.borrow().right.as_ref().unwrap().as_ptr() != black_node.as_ptr() {
                    let black_next = black_pre.borrow().right.clone().unwrap();
                    black_pre = black_next;
                }
                if black_pre.borrow().right.is_none() {
                    black_pre.borrow_mut().right = Some(black_node.clone());
                    black_curr = black_node.borrow().left.clone();
                } else {
                    black_pre.borrow_mut().right = None;
                    if let Some(ref black_prev) = black_p {
                        if black_prev.borrow().val > black_node.borrow().val {
                            if black_f.is_none() { black_f = black_p.clone(); }
                            black_s = Some(black_node.clone());
                        }
                    }
                    black_p = Some(black_node.clone());
                    black_curr = black_node.borrow().right.clone();
                }
            }
        }
        if let (Some(black_n1), Some(black_n2)) = (black_f, black_s) {
            std::mem::swap(&mut black_n1.borrow_mut().val, &mut black_n2.borrow_mut().val);
        }
    }
}
```