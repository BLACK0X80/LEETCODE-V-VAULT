# Binary Tree Cameras

**Difficulty:** Hard
**Tags:** Dynamic Programming, Tree, Depth-First Search, Binary Tree

---

## Problem

<p>You are given the <code>root</code> of a binary tree. We install cameras on the tree nodes where each camera at a node can monitor its parent, itself, and its immediate children.</p>

<p>Return <em>the minimum number of cameras needed to monitor all nodes of the tree</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_01.png" style="width: 138px; height: 163px;" />
<pre>
<strong>Input:</strong> root = [0,0,null,0,0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> One camera is enough to monitor all nodes if placed as shown.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_02.png" style="width: 139px; height: 312px;" />
<pre>
<strong>Input:</strong> root = [0,0,null,0,null,0,null,null,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong> At least two cameras are needed to monitor all nodes of the tree. The above image shows one of the valid configurations of camera placement.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 1000]</code>.</li>
	<li><code>Node.val == 0</code></li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cameras = 0;
        if dfs(&root, &mut cameras) == 0 { cameras += 1; }
        cameras
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, cameras: &mut i32) -> i32 {
    match node {
        None => 1,
        Some(n) => {
            let n = n.borrow();
            let l = dfs(&n.left, cameras);
            let r = dfs(&n.right, cameras);
            if l == 0 || r == 0 {
                *cameras += 1;
                return 2;
            }
            if l == 2 || r == 2 { return 1; }
            0
        }
    }
}
```