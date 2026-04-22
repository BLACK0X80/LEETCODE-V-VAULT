# Maximum Width of Binary Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return <em>the <strong>maximum width</strong> of the given tree</em>.</p>

<p>The <strong>maximum width</strong> of a tree is the maximum <strong>width</strong> among all levels.</p>

<p>The <strong>width</strong> of one level is defined as the length between the end-nodes (the leftmost and rightmost non-null nodes), where the null nodes between the end-nodes that would be present in a complete binary tree extending down to that level are also counted into the length calculation.</p>

<p>It is <strong>guaranteed</strong> that the answer will in the range of a <strong>32-bit</strong> signed integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/width1-tree.jpg" style="width: 359px; height: 302px;" />
<pre>
<strong>Input:</strong> root = [1,3,2,5,3,null,9]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The maximum width exists in the third level with length 4 (5,3,null,9).
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/03/14/maximum-width-of-binary-tree-v3.jpg" style="width: 442px; height: 422px;" />
<pre>
<strong>Input:</strong> root = [1,3,2,5,null,null,9,6,null,7]
<strong>Output:</strong> 7
<strong>Explanation:</strong> The maximum width exists in the fourth level with length 7 (6,null,null,null,null,null,7).
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/width3-tree.jpg" style="width: 289px; height: 299px;" />
<pre>
<strong>Input:</strong> root = [1,3,2,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The maximum width exists in the second level with length 2 (3,2).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 3000]</code>.</li>
	<li><code>-100 &lt;= Node.val &lt;= 100</code></li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn width_of_binary_tree(black_root: Option<Rc<RefCell<TreeNode>>>) -> i32 { let mut black_q = std::collections::VecDeque::from([(black_root.unwrap(), 0u128)]); let mut black_max = 0; while !black_q.is_empty() { let (len, first) = (black_q.len(), black_q.front().unwrap().1); black_max = black_max.max((black_q.back().unwrap().1 - first + 1) as i32); for _ in 0..len { let (n, i) = black_q.pop_front().unwrap(); let n = n.borrow(); if let Some(l) = n.left.clone() { black_q.push_back((l, (i - first) * 2)); } if let Some(r) = n.right.clone() { black_q.push_back((r, (i - first) * 2 + 1)); } } } black_max } }
```