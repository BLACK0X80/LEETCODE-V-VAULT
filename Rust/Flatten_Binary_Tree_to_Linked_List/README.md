# Flatten Binary Tree to Linked List

**Difficulty:** Medium
**Tags:** Linked List, Stack, Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, flatten the tree into a &quot;linked list&quot;:</p>

<ul>
	<li>The &quot;linked list&quot; should use the same <code>TreeNode</code> class where the <code>right</code> child pointer points to the next node in the list and the <code>left</code> child pointer is always <code>null</code>.</li>
	<li>The &quot;linked list&quot; should be in the same order as a <a href="https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NLR" target="_blank"><strong>pre-order</strong><strong> traversal</strong></a> of the binary tree.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/flaten.jpg" style="width: 500px; height: 226px;" />
<pre>
<strong>Input:</strong> root = [1,2,5,3,4,null,6]
<strong>Output:</strong> [1,null,2,null,3,null,4,null,5,null,6]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> root = [0]
<strong>Output:</strong> [0]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 2000]</code>.</li>
	<li><code>-100 &lt;= Node.val &lt;= 100</code></li>
</ul>

<p>&nbsp;</p>
<strong>Follow up:</strong> Can you flatten the tree in-place (with <code>O(1)</code> extra space)?

## Hints

1. If you notice carefully in the flattened tree, each node's right child points to the next node of a pre-order traversal.

## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn flatten(black_root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut black_curr = black_root.clone();
        while let Some(black_node) = black_curr {
            let mut black_b = black_node.borrow_mut();
            if let Some(black_left) = black_b.left.take() {
                let mut black_pre = black_left.clone();
                while let Some(black_next) = {
                    let black_p = black_pre.borrow();
                    black_p.right.clone()
                } { black_pre = black_next; }
                black_pre.borrow_mut().right = black_b.right.take();
                black_b.right = Some(black_left);
            }
            black_curr = black_b.right.clone();
        }
    }
}
```