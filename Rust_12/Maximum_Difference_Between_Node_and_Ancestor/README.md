# Maximum Difference Between Node and Ancestor

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, find the maximum value <code>v</code> for which there exist <strong>different</strong> nodes <code>a</code> and <code>b</code> where <code>v = |a.val - b.val|</code> and <code>a</code> is an ancestor of <code>b</code>.</p>

<p>A node <code>a</code> is an ancestor of <code>b</code> if either: any child of <code>a</code> is equal to <code>b</code>&nbsp;or any child of <code>a</code> is an ancestor of <code>b</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/09/tmp-tree.jpg" style="width: 400px; height: 390px;" />
<pre>
<strong>Input:</strong> root = [8,3,10,1,6,null,14,null,null,4,7,13]
<strong>Output:</strong> 7
<strong>Explanation: </strong>We have various ancestor-node differences, some of which are given below :
|8 - 3| = 5
|3 - 7| = 4
|8 - 1| = 7
|10 - 13| = 3
Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/09/tmp-tree-1.jpg" style="width: 250px; height: 349px;" />
<pre>
<strong>Input:</strong> root = [1,null,2,null,0,3]
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[2, 5000]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. For each subtree, find the minimum value and maximum value of its descendants.

## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn max_ancestor_diff(black_root: Option<Rc<RefCell<TreeNode>>>) -> i32 { fn black_dfs(black_node: Option<Rc<RefCell<TreeNode>>>, mut black_min: i32, mut black_max: i32) -> i32 { if let Some(black_n) = black_node { let black_v = black_n.borrow().val; black_min = black_min.min(black_v); black_max = black_max.max(black_v); black_dfs(black_n.borrow().left.clone(), black_min, black_max).max(black_dfs(black_n.borrow().right.clone(), black_min, black_max)) } else { black_max - black_min } } let black_v = black_root.as_ref().unwrap().borrow().val; black_dfs(black_root, black_v, black_v) } }
```