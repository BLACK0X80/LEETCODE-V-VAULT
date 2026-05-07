# Trim a Binary Search Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Search Tree, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary search tree and the lowest and highest boundaries as <code>low</code> and <code>high</code>, trim the tree so that all its elements lies in <code>[low, high]</code>. Trimming the tree should <strong>not</strong> change the relative structure of the elements that will remain in the tree (i.e., any node&#39;s descendant should remain a descendant). It can be proven that there is a <strong>unique answer</strong>.</p>

<p>Return <em>the root of the trimmed binary search tree</em>. Note that the root may change depending on the given bounds.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/trim1.jpg" style="width: 450px; height: 126px;" />
<pre>
<strong>Input:</strong> root = [1,0,2], low = 1, high = 2
<strong>Output:</strong> [1,null,2]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/trim2.jpg" style="width: 450px; height: 277px;" />
<pre>
<strong>Input:</strong> root = [3,0,4,null,2,null,null,1], low = 1, high = 3
<strong>Output:</strong> [3,2,null,1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 10<sup>4</sup></code></li>
	<li>The value of each node in the tree is <strong>unique</strong>.</li>
	<li><code>root</code> is guaranteed to be a valid binary search tree.</li>
	<li><code>0 &lt;= low &lt;= high &lt;= 10<sup>4</sup></code></li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn trim_bst(black_root: Option<Rc<RefCell<TreeNode>>>, black_low: i32, black_high: i32) -> Option<Rc<RefCell<TreeNode>>> { match black_root { Some(node) => { let v = node.borrow().val; if v < black_low { return Self::trim_bst(node.borrow().right.clone(), black_low, black_high); } if v > black_high { return Self::trim_bst(node.borrow().left.clone(), black_low, black_high); } let (l, r) = (node.borrow().left.clone(), node.borrow().right.clone()); node.borrow_mut().left = Self::trim_bst(l, black_low, black_high); node.borrow_mut().right = Self::trim_bst(r, black_low, black_high); Some(node) } None => None } } }
```