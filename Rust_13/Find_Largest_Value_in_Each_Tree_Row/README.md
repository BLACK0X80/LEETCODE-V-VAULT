# Find Largest Value in Each Tree Row

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return <em>an array of the largest value in each row</em> of the tree <strong>(0-indexed)</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/08/21/largest_e1.jpg" style="width: 300px; height: 172px;" />
<pre>
<strong>Input:</strong> root = [1,3,2,5,3,null,9]
<strong>Output:</strong> [1,3,9]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [1,2,3]
<strong>Output:</strong> [1,3]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree will be in the range <code>[0, 10<sup>4</sup>]</code>.</li>
	<li><code>-2<sup>31</sup> &lt;= Node.val &lt;= 2<sup>31</sup> - 1</code></li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn largest_values(black_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> { let mut black_res = vec![]; if black_root.is_none() { return black_res; } let mut black_q = std::collections::VecDeque::from([black_root.unwrap()]); while !black_q.is_empty() { let mut black_max = i32::MIN; for _ in 0..black_q.len() { let n = black_q.pop_front().unwrap(); let n = n.borrow(); black_max = black_max.max(n.val); if let Some(l) = n.left.clone() { black_q.push_back(l); } if let Some(r) = n.right.clone() { black_q.push_back(r); } } black_res.push(black_max); } black_res } }
```