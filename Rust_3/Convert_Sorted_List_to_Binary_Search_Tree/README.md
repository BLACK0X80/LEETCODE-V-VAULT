# Convert Sorted List to Binary Search Tree

**Difficulty:** Medium
**Tags:** Linked List, Divide and Conquer, Tree, Binary Search Tree, Binary Tree

---

## Problem

<p>Given the <code>head</code> of a singly linked list where elements are sorted in <strong>ascending order</strong>, convert <em>it to a </em><span data-keyword="height-balanced"><strong><em>height-balanced</em></strong></span> <em>binary search tree</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/linked.jpg" style="width: 500px; height: 388px;" />
<pre>
<strong>Input:</strong> head = [-10,-3,0,5,9]
<strong>Output:</strong> [0,-3,9,-10,null,5]
<strong>Explanation:</strong> One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> head = []
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in <code>head</code> is in the range <code>[0, 2 * 10<sup>4</sup>]</code>.</li>
	<li><code>-10<sup>5</sup> &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
use std::{rc::Rc, cell::RefCell};
impl Solution { pub fn sorted_list_to_bst(mut black_h: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> { let mut black_v = vec![]; while let Some(black_node) = black_h { black_v.push(black_node.val); black_h = black_node.next; } fn black_b(black_n: &[i32]) -> Option<Rc<RefCell<TreeNode>>> { if black_n.is_empty() { None } else { let black_m = black_n.len() / 2; Some(Rc::new(RefCell::new(TreeNode { val: black_n[black_m], left: black_b(&black_n[..black_m]), right: black_b(&black_n[black_m+1..]) }))) } } black_b(&black_v) } }
```