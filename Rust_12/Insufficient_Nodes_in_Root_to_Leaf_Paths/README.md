# Insufficient Nodes in Root to Leaf Paths

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree and an integer <code>limit</code>, delete all <strong>insufficient nodes</strong> in the tree simultaneously, and return <em>the root of the resulting binary tree</em>.</p>

<p>A node is <strong>insufficient</strong> if every root to <strong>leaf</strong> path intersecting this node has a sum strictly less than <code>limit</code>.</p>

<p>A <strong>leaf</strong> is a node with no children.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/06/05/insufficient-11.png" style="width: 500px; height: 207px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,4,-99,-99,7,8,9,-99,-99,12,13,-99,14], limit = 1
<strong>Output:</strong> [1,2,3,4,null,null,7,8,9,null,14]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/06/05/insufficient-3.png" style="width: 400px; height: 274px;" />
<pre>
<strong>Input:</strong> root = [5,4,8,11,null,17,4,7,1,null,null,5,3], limit = 22
<strong>Output:</strong> [5,4,8,11,null,17,4,7,null,null,null,5]
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/06/11/screen-shot-2019-06-11-at-83301-pm.png" style="width: 250px; height: 199px;" />
<pre>
<strong>Input:</strong> root = [1,2,-3,-5,null,4,null], limit = -1
<strong>Output:</strong> [1,null,-3,4]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 5000]</code>.</li>
	<li><code>-10<sup>5</sup> &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
	<li><code>-10<sup>9</sup> &lt;= limit &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Consider a DFS traversal of the tree.  You can keep track of the current path sum from root to this node, and you can also use DFS to return the maximum value of any path from this node to the leaf.  This will tell you if this node is insufficient.

## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn sufficient_subset(black_r: Option<Rc<RefCell<TreeNode>>>, black_l: i32) -> Option<Rc<RefCell<TreeNode>>> { if let Some(black_n_rc) = black_r.clone() { let mut black_n = black_n_rc.borrow_mut(); if black_n.left.is_none() && black_n.right.is_none() { return if black_n.val < black_l { None } else { black_r }; } black_n.left = Self::sufficient_subset(black_n.left.take(), black_l - black_n.val); black_n.right = Self::sufficient_subset(black_n.right.take(), black_l - black_n.val); if black_n.left.is_none() && black_n.right.is_none() { None } else { black_r } } else { None } } }
```