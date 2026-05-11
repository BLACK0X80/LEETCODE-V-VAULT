# Sum of Nodes with Even-Valued Grandparent

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return <em>the sum of values of nodes with an <strong>even-valued grandparent</strong></em>. If there are no nodes with an <strong>even-valued grandparent</strong>, return <code>0</code>.</p>

<p>A <strong>grandparent</strong> of a node is the parent of its parent if it exists.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/even1-tree.jpg" style="width: 504px; height: 302px;" />
<pre>
<strong>Input:</strong> root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
<strong>Output:</strong> 18
<strong>Explanation:</strong> The red nodes are the nodes with even-value grandparent while the blue nodes are the even-value grandparents.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/even2-tree.jpg" style="width: 64px; height: 65px;" />
<pre>
<strong>Input:</strong> root = [1]
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 100</code></li>
</ul>


## Hints

1. Traverse the tree keeping the parent and the grandparent.
2. If the grandparent of the current node is even-valued, add the value of this node to the answer.

## Solution

```rust
impl Solution { pub fn sum_even_grandparent(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> i32 { fn black_dfs(n: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, black_p: i32, black_gp: i32) -> i32 { if let Some(black_node) = n { let black_v = black_node.borrow().val; (if black_gp % 2 == 0 {black_v} else {0}) + black_dfs(black_node.borrow().left.clone(), black_v, black_p) + black_dfs(black_node.borrow().right.clone(), black_v, black_p) } else { 0 } } black_dfs(root, 1, 1) } }
```