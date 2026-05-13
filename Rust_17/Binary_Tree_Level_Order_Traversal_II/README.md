# Binary Tree Level Order Traversal II

**Difficulty:** Medium
**Tags:** Tree, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return <em>the bottom-up level order traversal of its nodes&#39; values</em>. (i.e., from left to right, level by level from leaf to root).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
<pre>
<strong>Input:</strong> root = [3,9,20,null,null,15,7]
<strong>Output:</strong> [[15,7],[9,20],[3]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [1]
<strong>Output:</strong> [[1]]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 2000]</code>.</li>
	<li><code>-1000 &lt;= Node.val &lt;= 1000</code></li>
</ul>



## Solution

```rust
use std::{rc::Rc, cell::RefCell};
impl Solution { pub fn level_order_bottom(black_r: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> { let mut black_res = vec![]; fn black_dfs(black_n: &Option<Rc<RefCell<TreeNode>>>, black_l: usize, black_res: &mut Vec<Vec<i32>>) { if let Some(black_node) = black_n { if black_res.len() == black_l { black_res.push(vec![]); } black_res[black_l].push(black_node.borrow().val); black_dfs(&black_node.borrow().left, black_l + 1, black_res); black_dfs(&black_node.borrow().right, black_l + 1, black_res); } } black_dfs(&black_r, 0, &mut black_res); black_res.reverse(); black_res } }
```