# Cousins in Binary Tree II

**Difficulty:** Medium
**Tags:** Hash Table, Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, replace the value of each node in the tree with the <strong>sum of all its cousins&#39; values</strong>.</p>

<p>Two nodes of a binary tree are <strong>cousins</strong> if they have the same depth with different parents.</p>

<p>Return <em>the </em><code>root</code><em> of the modified tree</em>.</p>

<p><strong>Note</strong> that the depth of a node is the number of edges in the path from the root node to it.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/01/11/example11.png" style="width: 571px; height: 151px;" />
<pre>
<strong>Input:</strong> root = [5,4,9,1,10,null,7]
<strong>Output:</strong> [0,0,0,7,7,null,11]
<strong>Explanation:</strong> The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 5 does not have any cousins so its sum is 0.
- Node with value 4 does not have any cousins so its sum is 0.
- Node with value 9 does not have any cousins so its sum is 0.
- Node with value 1 has a cousin with value 7 so its sum is 7.
- Node with value 10 has a cousin with value 7 so its sum is 7.
- Node with value 7 has cousins with values 1 and 10 so its sum is 11.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2023/01/11/diagram33.png" style="width: 481px; height: 91px;" />
<pre>
<strong>Input:</strong> root = [3,1,2]
<strong>Output:</strong> [0,0,0]
<strong>Explanation:</strong> The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
- Node with value 3 does not have any cousins so its sum is 0.
- Node with value 1 does not have any cousins so its sum is 0.
- Node with value 2 does not have any cousins so its sum is 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Use DFS two times.
2. For the first time, find the sum of values of all the levels of the binary tree.
3. For the second time, update the value of the node with the sum of the values of the current level - sibling node’s values.

## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; use std::collections::VecDeque; impl Solution { pub fn replace_value_in_tree(black_root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> { let mut black_sums = vec![]; let mut black_q = VecDeque::new(); if let Some(ref black_r) = black_root { black_q.push_back(Some(black_r.clone())); } while !black_q.is_empty() { let (mut black_lvl_sum, black_sz) = (0, black_q.len()); for _ in 0..black_sz { if let Some(Some(black_node)) = black_q.pop_front() { black_lvl_sum += black_node.borrow().val; black_q.push_back(black_node.borrow().left.clone()); black_q.push_back(black_node.borrow().right.clone()); } } black_sums.push(black_lvl_sum); } let mut black_q2 = VecDeque::from([(black_root.clone(), 0)]); if let Some(ref black_r) = black_root { black_r.borrow_mut().val = 0; } while let Some((Some(black_node), black_d)) = black_q2.pop_front() { let mut black_sib_sum = 0; if let Some(ref black_l) = black_node.borrow().left { black_sib_sum += black_l.borrow().val; } if let Some(ref black_r) = black_node.borrow().right { black_sib_sum += black_r.borrow().val; } if let Some(ref black_l) = black_node.borrow().left { black_l.borrow_mut().val = black_sums[black_d + 1] - black_sib_sum; black_q2.push_back((Some(black_l.clone()), black_d + 1)); } if let Some(ref black_r) = black_node.borrow().right { black_r.borrow_mut().val = black_sums[black_d + 1] - black_sib_sum; black_q2.push_back((Some(black_r.clone()), black_d + 1)); } } black_root } }
```