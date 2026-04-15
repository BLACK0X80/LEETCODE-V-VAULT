# Binary Tree Zigzag Level Order Traversal

**Difficulty:** Medium
**Tags:** Tree, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return <em>the zigzag level order traversal of its nodes&#39; values</em>. (i.e., from left to right, then right to left for the next level and alternate between).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
<pre>
<strong>Input:</strong> root = [3,9,20,null,null,15,7]
<strong>Output:</strong> [[3],[20,9],[15,7]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> root = [1]
<strong>Output:</strong> [[1]]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 2000]</code>.</li>
	<li><code>-100 &lt;= Node.val &lt;= 100</code></li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn zigzag_level_order(black_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut black_res = vec![];
        fn black_dfs(black_node: &Option<Rc<RefCell<TreeNode>>>, black_lvl: usize, black_res: &mut Vec<Vec<i32>>) {
            if let Some(black_n) = black_node {
                if black_res.len() == black_lvl { black_res.push(vec![]); }
                black_res[black_lvl].push(black_n.borrow().val);
                black_dfs(&black_n.borrow().left, black_lvl + 1, black_res);
                black_dfs(&black_n.borrow().right, black_lvl + 1, black_res);
            }
        }
        black_dfs(&black_root, 0, &mut black_res);
        for black_i in 0..black_res.len() {
            if black_i % 2 == 1 { black_res[black_i].reverse(); }
        }
        black_res
    }
}
```