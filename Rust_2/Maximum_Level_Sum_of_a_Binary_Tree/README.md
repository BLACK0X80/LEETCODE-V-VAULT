# Maximum Level Sum of a Binary Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, the level of its root is <code>1</code>, the level of its children is <code>2</code>, and so on.</p>

<p>Return the <strong>smallest</strong> level <code>x</code> such that the sum of all the values of nodes at level <code>x</code> is <strong>maximal</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/capture.JPG" style="width: 200px; height: 175px;" />
<pre>
<strong>Input:</strong> root = [1,7,0,7,-8,null,null]
<strong>Output:</strong> 2
<strong>Explanation: </strong>
Level 1 sum = 1.
Level 2 sum = 7 + 0 = 7.
Level 3 sum = 7 + -8 = -1.
So we return the level with the maximum sum which is level 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [989,null,10250,98693,-89388,null,null,null,-32127]
<strong>Output:</strong> 2
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>-10<sup>5</sup> &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Calculate the sum for each level then find the level with the maximum sum.
2. How can you traverse the tree ?
3. How can you sum up the values for every level ?
4. Use DFS or BFS to traverse the tree keeping the level of each node, and sum up those values with a map or a frequency array.

## Solution

```rust
impl Solution { pub fn max_level_sum(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> i32 { let (mut black_q, mut black_max, mut black_res, mut black_l) = (std::collections::VecDeque::from([root.unwrap()]), i32::MIN, 0, 1); while !black_q.is_empty() { let mut black_cur = 0; for _ in 0..black_q.len() { let black_n = black_q.pop_front().unwrap(); black_cur += black_n.borrow().val; if let Some(black_left) = black_n.borrow().left.clone() { black_q.push_back(black_left); } if let Some(black_right) = black_n.borrow().right.clone() { black_q.push_back(black_right); } } if black_cur > black_max { black_max = black_cur; black_res = black_l; } black_l += 1; } black_res } }
```