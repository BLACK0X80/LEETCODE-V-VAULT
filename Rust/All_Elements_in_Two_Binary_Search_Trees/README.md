# All Elements in Two Binary Search Trees

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Search Tree, Sorting, Binary Tree

---

## Problem

<p>Given two binary search trees <code>root1</code> and <code>root2</code>, return <em>a list containing all the integers from both trees sorted in <strong>ascending</strong> order</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/12/18/q2-e1.png" style="width: 457px; height: 207px;" />
<pre>
<strong>Input:</strong> root1 = [2,1,4], root2 = [1,0,3]
<strong>Output:</strong> [0,1,1,2,3,4]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/12/18/q2-e5-.png" style="width: 352px; height: 197px;" />
<pre>
<strong>Input:</strong> root1 = [1,null,8], root2 = [8,1]
<strong>Output:</strong> [1,1,8,8]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in each tree is in the range <code>[0, 5000]</code>.</li>
	<li><code>-10<sup>5</sup> &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Traverse the first tree in list1 and the second tree in list2.
2. Merge the two trees in one list and sort it.

## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn get_all_elements(black_r1: Option<Rc<RefCell<TreeNode>>>, black_r2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> { let mut black_v = vec![]; fn black_dfs(black_n: Option<Rc<RefCell<TreeNode>>>, black_res: &mut Vec<i32>) { if let Some(black_node) = black_n { black_dfs(black_node.borrow().left.clone(), black_res); black_res.push(black_node.borrow().val); black_dfs(black_node.borrow().right.clone(), black_res); } } black_dfs(black_r1, &mut black_v); black_dfs(black_r2, &mut black_v); black_v.sort_unstable(); black_v } }
```