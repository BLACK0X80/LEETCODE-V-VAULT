# Longest Univalue Path

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return <em>the length of the longest path, where each node in the path has the same value</em>. This path may or may not pass through the root.</p>

<p><strong>The length of the path</strong> between two nodes is represented by the number of edges between them.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/ex1.jpg" style="width: 450px; height: 238px;" />
<pre>
<strong>Input:</strong> root = [5,4,5,1,1,null,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The shown image shows that the longest path of the same value (i.e. 5).
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/ex2.jpg" style="width: 450px; height: 238px;" />
<pre>
<strong>Input:</strong> root = [1,4,5,4,4,null,5]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The shown image shows that the longest path of the same value (i.e. 4).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 10<sup>4</sup>]</code>.</li>
	<li><code>-1000 &lt;= Node.val &lt;= 1000</code></li>
	<li>The depth of the tree will not exceed <code>1000</code>.</li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        let node = match node { Some(n) => n, None => return 0 };
        let n = node.borrow();
        let l = Self::dfs(&n.left, ans);
        let r = Self::dfs(&n.right, ans);
        let lp = if n.left.as_ref().map_or(false, |x| x.borrow().val == n.val) { l + 1 } else { 0 };
        let rp = if n.right.as_ref().map_or(false, |x| x.borrow().val == n.val) { r + 1 } else { 0 };
        *ans = (*ans).max(lp + rp);
        lp.max(rp)
    }
}
```