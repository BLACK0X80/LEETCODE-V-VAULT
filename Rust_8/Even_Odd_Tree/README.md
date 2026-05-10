# Even Odd Tree

**Difficulty:** Medium
**Tags:** Tree, Breadth-First Search, Binary Tree

---

## Problem

<p>A binary tree is named <strong>Even-Odd</strong> if it meets the following conditions:</p>

<ul>
	<li>The root of the binary tree is at level index <code>0</code>, its children are at level index <code>1</code>, their children are at level index <code>2</code>, etc.</li>
	<li>For every <strong>even-indexed</strong> level, all nodes at the level have <strong>odd</strong> integer values in <strong>strictly increasing</strong> order (from left to right).</li>
	<li>For every <b>odd-indexed</b> level, all nodes at the level have <b>even</b> integer values in <strong>strictly decreasing</strong> order (from left to right).</li>
</ul>

<p>Given the <code>root</code> of a binary tree, <em>return </em><code>true</code><em> if the binary tree is <strong>Even-Odd</strong>, otherwise return </em><code>false</code><em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/sample_1_1966.png" style="width: 362px; height: 229px;" />
<pre>
<strong>Input:</strong> root = [1,10,4,3,null,7,9,12,8,6,null,null,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> The node values on each level are:
Level 0: [1]
Level 1: [10,4]
Level 2: [3,7,9]
Level 3: [12,8,6,2]
Since levels 0 and 2 are all odd and increasing and levels 1 and 3 are all even and decreasing, the tree is Even-Odd.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/sample_2_1966.png" style="width: 363px; height: 167px;" />
<pre>
<strong>Input:</strong> root = [5,4,2,3,3,7]
<strong>Output:</strong> false
<strong>Explanation:</strong> The node values on each level are:
Level 0: [5]
Level 1: [4,2]
Level 2: [3,3,7]
Node values in level 2 must be in strictly increasing order, so the tree is not Even-Odd.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/22/sample_1_333_1966.png" style="width: 363px; height: 167px;" />
<pre>
<strong>Input:</strong> root = [5,9,1,3,5,7]
<strong>Output:</strong> false
<strong>Explanation:</strong> Node values in the level 1 should be even integers.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Use the breadth-first search to go through all nodes layer by layer.

## Solution

```rust
impl Solution { pub fn is_even_odd_tree(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> bool { let (mut black_q, mut black_level) = (std::collections::VecDeque::from([root.unwrap()]), 0); while !black_q.is_empty() { let (mut black_prev, black_len) = (if black_level % 2 == 0 { i32::MIN } else { i32::MAX }, black_q.len()); for _ in 0..black_len { let black_node = black_q.pop_front().unwrap(); let black_val = black_node.borrow().val; if black_level % 2 == 0 { if black_val % 2 == 0 || black_val <= black_prev { return false; } } else { if black_val % 2 != 0 || black_val >= black_prev { return false; } } black_prev = black_val; if let Some(black_l) = black_node.borrow().left.clone() { black_q.push_back(black_l); } if let Some(black_r) = black_node.borrow().right.clone() { black_q.push_back(black_r); } } black_level += 1; } true } }
```