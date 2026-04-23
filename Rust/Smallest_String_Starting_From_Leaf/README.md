# Smallest String Starting From Leaf

**Difficulty:** Medium
**Tags:** String, Backtracking, Tree, Depth-First Search, Binary Tree

---

## Problem

<p>You are given the <code>root</code> of a binary tree where each node has a value in the range <code>[0, 25]</code> representing the letters <code>&#39;a&#39;</code> to <code>&#39;z&#39;</code>.</p>

<p>Return <em>the <strong>lexicographically smallest</strong> string that starts at a leaf of this tree and ends at the root</em>.</p>

<p>As a reminder, any shorter prefix of a string is <strong>lexicographically smaller</strong>.</p>

<ul>
	<li>For example, <code>&quot;ab&quot;</code> is lexicographically smaller than <code>&quot;aba&quot;</code>.</li>
</ul>

<p>A leaf of a node is a node that has no children.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/01/30/tree1.png" style="width: 534px; height: 358px;" />
<pre>
<strong>Input:</strong> root = [0,1,2,3,4,3,4]
<strong>Output:</strong> &quot;dba&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/01/30/tree2.png" style="width: 534px; height: 358px;" />
<pre>
<strong>Input:</strong> root = [25,1,3,1,3,0,2]
<strong>Output:</strong> &quot;adz&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/02/01/tree3.png" style="height: 490px; width: 468px;" />
<pre>
<strong>Input:</strong> root = [2,2,1,null,1,0,null,0]
<strong>Output:</strong> &quot;abc&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 8500]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 25</code></li>
</ul>



## Solution

```rust
use std::{rc::Rc, cell::RefCell}; impl Solution { pub fn smallest_from_leaf(black_r: Option<Rc<RefCell<TreeNode>>>) -> String { let mut black_res = "~".to_string(); fn black_dfs(n: Option<Rc<RefCell<TreeNode>>>, mut black_s: String, black_res: &mut String) { if let Some(black_node) = n { let black_v = black_node.borrow(); black_s.push((b'a' + black_v.val as u8) as char); if black_v.left.is_none() && black_v.right.is_none() { let black_rev = black_s.chars().rev().collect::<String>(); if black_rev < *black_res { *black_res = black_rev; } } black_dfs(black_v.left.clone(), black_s.clone(), black_res); black_dfs(black_v.right.clone(), black_s, black_res); } } black_dfs(black_r, String::new(), &mut black_res); black_res } }
```