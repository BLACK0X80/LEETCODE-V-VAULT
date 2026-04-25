# Delete Nodes And Return Forest

**Difficulty:** Medium
**Tags:** Array, Hash Table, Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, each node in the tree has a distinct value.</p>

<p>After deleting all nodes with a value in <code>to_delete</code>, we are left with a forest (a disjoint union of trees).</p>

<p>Return the roots of the trees in the remaining forest. You may return the result in any order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/07/01/screen-shot-2019-07-01-at-53836-pm.png" style="width: 237px; height: 150px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6,7], to_delete = [3,5]
<strong>Output:</strong> [[1,2,null,4],[6],[7]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [1,2,4,null,3], to_delete = [3]
<strong>Output:</strong> [[1,2,4]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the given tree is at most <code>1000</code>.</li>
	<li>Each node has a distinct value between <code>1</code> and <code>1000</code>.</li>
	<li><code>to_delete.length &lt;= 1000</code></li>
	<li><code>to_delete</code> contains distinct values between <code>1</code> and <code>1000</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn del_nodes(black_root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, black_del: Vec<i32>) -> Vec<Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>> { let black_set: std::collections::HashSet<_> = black_del.into_iter().collect(); let mut black_res = vec![]; fn black_dfs(n: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, black_s: &std::collections::HashSet<i32>, black_r: &mut Vec<Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>>, is_root: bool) -> Option<std::rc::Rc<std::cell::RefCell<TreeNode>>> { n.as_ref().and_then(|black_node| { let black_val = black_node.borrow().val; let black_deleted = black_s.contains(&black_val); if is_root && !black_deleted { black_r.push(Some(black_node.clone())); } let (l, r) = (black_node.borrow().left.clone(), black_node.borrow().right.clone()); black_node.borrow_mut().left = black_dfs(l, black_s, black_r, black_deleted); black_node.borrow_mut().right = black_dfs(r, black_s, black_r, black_deleted); if black_deleted { None } else { Some(black_node.clone()) } }) } black_dfs(black_root, &black_set, &mut black_res, true); black_res } }
```