# Unique Binary Search Trees II

**Difficulty:** Medium
**Tags:** Dynamic Programming, Backtracking, Tree, Binary Search Tree, Binary Tree

---

## Problem

<p>Given an integer <code>n</code>, return <em>all the structurally unique <strong>BST&#39;</strong>s (binary search trees), which has exactly </em><code>n</code><em> nodes of unique values from</em> <code>1</code> <em>to</em> <code>n</code>. Return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> [[1]]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 8</code></li>
</ul>



## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 { return vec![]; }
        Self::black_build(1, n)
    }

    fn black_build(black_start: i32, black_end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut black_res = Vec::new();
        if black_start > black_end {
            black_res.push(None);
            return black_res;
        }

        for black_val in black_start..=black_end {
            let black_left_trees = Self::black_build(black_start, black_val - 1);
            let black_right_trees = Self::black_build(black_val + 1, black_end);

            for black_left in &black_left_trees {
                for black_right in &black_right_trees {
                    let black_root = Rc::new(RefCell::new(TreeNode::new(black_val)));
                    black_root.borrow_mut().left = black_left.clone();
                    black_root.borrow_mut().right = black_right.clone();
                    black_res.push(Some(black_root));
                }
            }
        }
        black_res
    }
}
```