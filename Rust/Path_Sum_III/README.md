# Path Sum III

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree and an integer <code>targetSum</code>, return <em>the number of paths where the sum of the values&nbsp;along the path equals</em>&nbsp;<code>targetSum</code>.</p>

<p>The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/pathsum3-1-tree.jpg" style="width: 450px; height: 386px;" />
<pre>
<strong>Input:</strong> root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
<strong>Output:</strong> 3
<strong>Explanation:</strong> The paths that sum to 8 are shown.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 1000]</code>.</li>
	<li><code>-10<sup>9</sup> &lt;= Node.val &lt;= 10<sup>9</sup></code></li>
	<li><code>-1000 &lt;= targetSum &lt;= 1000</code></li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn path_sum(black_root: Option<Rc<RefCell<TreeNode>>>, black_target: i32) -> i32 { let mut black_map = std::collections::HashMap::from([(0i64, 1)]); fn dfs(node: Option<Rc<RefCell<TreeNode>>>, curr: i64, target: i32, map: &mut std::collections::HashMap<i64, i32>) -> i32 { if let Some(n) = node { let n = n.borrow(); let new_curr = curr + n.val as i64; let mut res = *map.get(&(new_curr - target as i64)).unwrap_or(&0); *map.entry(new_curr).or_insert(0) += 1; res += dfs(n.left.clone(), new_curr, target, map) + dfs(n.right.clone(), new_curr, target, map); *map.get_mut(&new_curr).unwrap() -= 1; res } else { 0 } } dfs(black_root, 0, black_target, &mut black_map) } }
```