# Kth Smallest Element in a BST

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Binary Search Tree, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary search tree, and an integer <code>k</code>, return <em>the</em> <code>k<sup>th</sup></code> <em>smallest value (<strong>1-indexed</strong>) of all the values of the nodes in the tree</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree1.jpg" style="width: 212px; height: 301px;" />
<pre>
<strong>Input:</strong> root = [3,1,4,null,2], k = 1
<strong>Output:</strong> 1
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree2.jpg" style="width: 382px; height: 302px;" />
<pre>
<strong>Input:</strong> root = [5,3,6,2,4,null,null,1], k = 3
<strong>Output:</strong> 3
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is <code>n</code>.</li>
	<li><code>1 &lt;= k &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= Node.val &lt;= 10<sup>4</sup></code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> If the BST is modified often (i.e., we can do insert and delete operations) and you need to find the kth smallest frequently, how would you optimize?</p>


## Hints

1. Try to utilize the property of a BST.
2. Try in-order traversal. (Credits to @chan13)
3. What if you could modify the BST node's structure?
4. The optimal runtime complexity is O(height of BST).

## Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution { pub fn kth_smallest(black_r: Option<Rc<RefCell<TreeNode>>>, mut black_k: i32) -> i32 { let mut black_res = 0; Self::black_dfs(black_r, &mut black_k, &mut black_res); black_res } fn black_dfs(black_n: Option<Rc<RefCell<TreeNode>>>, black_k: &mut i32, black_res: &mut i32) { if let Some(black_node) = black_n { Self::black_dfs(black_node.borrow().left.clone(), black_k, black_res); *black_k -= 1; if *black_k == 0 { *black_res = black_node.borrow().val; return; } if *black_k > 0 { Self::black_dfs(black_node.borrow().right.clone(), black_k, black_res); } } } }
```