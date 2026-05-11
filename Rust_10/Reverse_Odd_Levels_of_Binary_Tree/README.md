# Reverse Odd Levels of Binary Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a <strong>perfect</strong> binary tree, reverse the node values at each <strong>odd</strong> level of the tree.</p>

<ul>
	<li>For example, suppose the node values at level 3 are <code>[2,1,3,4,7,11,29,18]</code>, then it should become <code>[18,29,11,7,4,3,1,2]</code>.</li>
</ul>

<p>Return <em>the root of the reversed tree</em>.</p>

<p>A binary tree is <strong>perfect</strong> if all parent nodes have two children and all leaves are on the same level.</p>

<p>The <strong>level</strong> of a node is the number of edges along the path between it and the root node.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/07/28/first_case1.png" style="width: 626px; height: 191px;" />
<pre>
<strong>Input:</strong> root = [2,3,5,8,13,21,34]
<strong>Output:</strong> [2,5,3,8,13,21,34]
<strong>Explanation:</strong> 
The tree has only one odd level.
The nodes at level 1 are 3, 5 respectively, which are reversed and become 5, 3.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/07/28/second_case3.png" style="width: 591px; height: 111px;" />
<pre>
<strong>Input:</strong> root = [7,13,11]
<strong>Output:</strong> [7,11,13]
<strong>Explanation:</strong> 
The nodes at level 1 are 13, 11, which are reversed and become 11, 13.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> root = [0,1,2,0,0,0,0,1,1,1,1,2,2,2,2]
<strong>Output:</strong> [0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]
<strong>Explanation:</strong> 
The odd levels have non-zero values.
The nodes at level 1 were 1, 2, and are 2, 1 after the reversal.
The nodes at level 3 were 1, 1, 1, 1, 2, 2, 2, 2, and are 2, 2, 2, 2, 1, 1, 1, 1 after the reversal.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 2<sup>14</sup>]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
	<li><code>root</code> is a <strong>perfect</strong> binary tree.</li>
</ul>


## Hints

1. Try to solve recursively for each level independently.
2. While performing a depth-first search, pass the left and right nodes (which should be paired) to the next level. If the current level is odd, then reverse their values, or else recursively move to the next level.

## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; use std::collections::VecDeque; impl Solution { pub fn reverse_odd_levels(black_root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> { let mut black_q = VecDeque::new(); if let Some(ref black_r) = black_root { black_q.push_back(black_r.clone()); } let mut black_lvl = 0; while !black_q.is_empty() { let black_sz = black_q.len(); if black_lvl % 2 == 1 { let mut black_vals: Vec<_> = black_q.iter().map(|n| n.borrow().val).collect(); black_vals.reverse(); for (black_i, black_node) in black_q.iter().enumerate() { black_node.borrow_mut().val = black_vals[black_i]; } } for _ in 0..black_sz { let black_curr = black_q.pop_front().unwrap(); if let Some(l) = black_curr.borrow().left.clone() { black_q.push_back(l); } if let Some(r) = black_curr.borrow().right.clone() { black_q.push_back(r); } } black_lvl += 1; } black_root } }
```