# Symmetric Tree

**Difficulty:** Easy
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, <em>check whether it is a mirror of itself</em> (i.e., symmetric around its center).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree1.jpg" style="width: 354px; height: 291px;" />
<pre>
<strong>Input:</strong> root = [1,2,2,3,4,4,3]
<strong>Output:</strong> true
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree2.jpg" style="width: 308px; height: 258px;" />
<pre>
<strong>Input:</strong> root = [1,2,2,null,3,null,3]
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 1000]</code>.</li>
	<li><code>-100 &lt;= Node.val &lt;= 100</code></li>
</ul>

<p>&nbsp;</p>
<strong>Follow up:</strong> Could you solve it both recursively and iteratively?


## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool { Self::black_check(root.as_ref().and_then(|black_n| black_n.borrow().left.clone()), root.as_ref().and_then(|black_n| black_n.borrow().right.clone())) } fn black_check(black_p: Option<Rc<RefCell<TreeNode>>>, black_q: Option<Rc<RefCell<TreeNode>>>) -> bool { match (black_p, black_q) { (None, None) => true, (Some(black_l), Some(black_r)) => { let (black_n1, black_n2) = (black_l.borrow(), black_r.borrow()); black_n1.val == black_n2.val && Self::black_check(black_n1.left.clone(), black_n2.right.clone()) && Self::black_check(black_n1.right.clone(), black_n2.left.clone()) } _ => false } } }
```