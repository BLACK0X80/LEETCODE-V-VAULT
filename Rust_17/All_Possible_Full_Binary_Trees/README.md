# All Possible Full Binary Trees

**Difficulty:** Medium
**Tags:** Dynamic Programming, Tree, Recursion, Memoization, Binary Tree

---

## Problem

<p>Given an integer <code>n</code>, return <em>a list of all possible <strong>full binary trees</strong> with</em> <code>n</code> <em>nodes</em>. Each node of each tree in the answer must have <code>Node.val == 0</code>.</p>

<p>Each element of the answer is the root node of one possible tree. You may return the final list of trees in <strong>any order</strong>.</p>

<p>A <strong>full binary tree</strong> is a binary tree where each node has exactly <code>0</code> or <code>2</code> children.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/22/fivetrees.png" style="width: 700px; height: 400px;" />
<pre>
<strong>Input:</strong> n = 7
<strong>Output:</strong> [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> [[0,0,0]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 20</code></li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn all_possible_fbt(black_n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> { if black_n % 2 == 0 { return vec![]; } if black_n == 1 { return vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))]; } let mut black_res = vec![]; for i in (1..black_n).step_by(2) { for black_left in Self::all_possible_fbt(i) { for black_right in Self::all_possible_fbt(black_n - 1 - i) { let mut black_root = TreeNode::new(0); black_root.left = black_left.clone(); black_root.right = black_right.clone(); black_res.push(Some(Rc::new(RefCell::new(black_root)))); } } } black_res } }
```