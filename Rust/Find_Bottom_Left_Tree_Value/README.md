# Find Bottom Left Tree Value

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return the leftmost value in the last row of the tree.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/tree1.jpg" style="width: 302px; height: 182px;" />
<pre>
<strong>Input:</strong> root = [2,1,3]
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/tree2.jpg" style="width: 432px; height: 421px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,4,null,5,6,null,null,7]
<strong>Output:</strong> 7
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>-2<sup>31</sup> &lt;= Node.val &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn find_bottom_left_value(black_root: Option<Rc<RefCell<TreeNode>>>) -> i32 { let mut black_q = std::collections::VecDeque::from([black_root.unwrap()]); let mut black_res = 0; while let Some(node) = black_q.pop_front() { let node = node.borrow(); black_res = node.val; if node.right.is_some() { black_q.push_back(node.right.clone().unwrap()); } if node.left.is_some() { black_q.push_back(node.left.clone().unwrap()); } } black_res } }
```