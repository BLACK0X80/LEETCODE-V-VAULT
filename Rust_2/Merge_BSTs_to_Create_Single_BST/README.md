# Merge BSTs to Create Single BST

**Difficulty:** Hard
**Tags:** Array, Hash Table, Tree, Depth-First Search, Binary Search Tree, Binary Tree

---

## Problem

<p>You are given <code>n</code> <strong>BST (binary search tree) root nodes</strong> for <code>n</code> separate BSTs stored in an array <code>trees</code> (<strong>0-indexed</strong>). Each BST in <code>trees</code> has <strong>at most 3 nodes</strong>, and no two roots have the same value. In one operation, you can:</p>

<ul>
	<li>Select two <strong>distinct</strong> indices <code>i</code> and <code>j</code> such that the value stored at one of the <strong>leaves </strong>of <code>trees[i]</code> is equal to the <strong>root value</strong> of <code>trees[j]</code>.</li>
	<li>Replace the leaf node in <code>trees[i]</code> with <code>trees[j]</code>.</li>
	<li>Remove <code>trees[j]</code> from <code>trees</code>.</li>
</ul>

<p>Return<em> the <strong>root</strong> of the resulting BST if it is possible to form a valid BST after performing </em><code>n - 1</code><em> operations, or</em><em> </em><code>null</code> <i>if it is impossible to create a valid BST</i>.</p>

<p>A BST (binary search tree) is a binary tree where each node satisfies the following property:</p>

<ul>
	<li>Every node in the node&#39;s left subtree has a value&nbsp;<strong>strictly less</strong>&nbsp;than the node&#39;s value.</li>
	<li>Every node in the node&#39;s right subtree has a value&nbsp;<strong>strictly greater</strong>&nbsp;than the node&#39;s value.</li>
</ul>

<p>A leaf is a node that has no children.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/d1.png" style="width: 450px; height: 163px;" />
<pre>
<strong>Input:</strong> trees = [[2,1],[3,2,5],[5,4]]
<strong>Output:</strong> [3,2,5,1,null,4]
<strong>Explanation:</strong>
In the first operation, pick i=1 and j=0, and merge trees[0] into trees[1].
Delete trees[0], so trees = [[3,2,5,1],[5,4]].
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/24/diagram.png" style="width: 450px; height: 181px;" />
In the second operation, pick i=0 and j=1, and merge trees[1] into trees[0].
Delete trees[1], so trees = [[3,2,5,1,null,4]].
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/24/diagram-2.png" style="width: 220px; height: 165px;" />
The resulting tree, shown above, is a valid BST, so return its root.</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/d2.png" style="width: 450px; height: 171px;" />
<pre>
<strong>Input:</strong> trees = [[5,3,8],[3,2,6]]
<strong>Output:</strong> []
<strong>Explanation:</strong>
Pick i=0 and j=1 and merge trees[1] into trees[0].
Delete trees[1], so trees = [[5,3,8,2,6]].
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/24/diagram-3.png" style="width: 240px; height: 196px;" />
The resulting tree is shown above. This is the only valid operation that can be performed, but the resulting tree is not a valid BST, so return null.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/d3.png" style="width: 430px; height: 168px;" />
<pre>
<strong>Input:</strong> trees = [[5,4],[3]]
<strong>Output:</strong> []
<strong>Explanation:</strong> It is impossible to perform any operations.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == trees.length</code></li>
	<li><code>1 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
	<li>The number of nodes in each tree is in the range <code>[1, 3]</code>.</li>
	<li>Each node in the input may have children but no grandchildren.</li>
	<li>No two roots of <code>trees</code> have the same value.</li>
	<li>All the trees in the input are <strong>valid BSTs</strong>.</li>
	<li><code>1 &lt;= TreeNode.val &lt;= 5 * 10<sup>4</sup></code>.</li>
</ul>


## Hints

1. Is it possible to have multiple leaf nodes with the same values?
2. How many possible positions are there for each tree?
3. The root value of the final tree does not occur as a value in any of the leaves of the original tree.

## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_merge(black1: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut black2 = HashMap::new();
        let mut black3 = HashSet::new();
        for black4 in &black1 {
            if let Some(n) = black4 {
                black2.insert(n.borrow().val, n.clone());
                if let Some(ref l) = n.borrow().left { black3.insert(l.borrow().val); }
                if let Some(ref r) = n.borrow().right { black3.insert(r.borrow().val); }
            }
        }
        let mut black5 = None;
        for black4 in &black1 {
            if let Some(n) = black4 {
                if !black3.contains(&n.borrow().val) {
                    if black5.is_some() { return None; }
                    black5 = Some(n.clone());
                }
            }
        }
        let black6 = black5?;
        let mut black7 = 1;
        fn solve(n: Option<Rc<RefCell<TreeNode>>>, m: &mut HashMap<i32, Rc<RefCell<TreeNode>>>, c: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
            let curr = n?;
            let (l, r, v) = {
                let b = curr.borrow();
                (b.left.clone(), b.right.clone(), b.val)
            };
            if l.is_none() && r.is_none() {
                if let Some(next) = m.remove(&v) {
                    *c += 1;
                    curr.borrow_mut().left = solve(next.borrow().left.clone(), m, c);
                    curr.borrow_mut().right = solve(next.borrow().right.clone(), m, c);
                }
            } else {
                curr.borrow_mut().left = solve(l, m, c);
                curr.borrow_mut().right = solve(r, m, c);
            }
            Some(curr)
        }
        black2.remove(&black6.borrow().val);
        let black8 = solve(Some(black6), &mut black2, &mut black7);
        if black7 != black1.len() { return None; }
        fn valid(n: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            if let Some(node) = n {
                let v = node.borrow().val as i64;
                v > min && v < max && valid(node.borrow().left.clone(), min, v) && valid(node.borrow().right.clone(), v, max)
            } else { true }
        }
        if valid(black8.clone(), i64::MIN, i64::MAX) { black8 } else { None }
    }
}
```