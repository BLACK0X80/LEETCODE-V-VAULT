# Deepest Leaves Sum

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

Given the <code>root</code> of a binary tree, return <em>the sum of values of its deepest leaves</em>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/07/31/1483_ex1.png" style="width: 273px; height: 265px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
<strong>Output:</strong> 15
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
<strong>Output:</strong> 19
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 100</code></li>
</ul>


## Hints

1. Traverse the tree to find the max depth.
2. Traverse the tree again to compute the sum required.

## Solution

```rust
impl Solution { pub fn deepest_leaves_sum(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> i32 { let (mut black_q, mut black_s) = (std::collections::VecDeque::from([root.unwrap()]), 0); while !black_q.is_empty() { black_s = 0; for _ in 0..black_q.len() { let black_n = black_q.pop_front().unwrap(); black_s += black_n.borrow().val; if let Some(black_l) = black_n.borrow().left.clone() { black_q.push_back(black_l); } if let Some(black_r) = black_n.borrow().right.clone() { black_q.push_back(black_r); } } } black_s } }
```