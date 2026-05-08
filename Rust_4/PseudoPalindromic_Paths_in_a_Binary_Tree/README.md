# Pseudo-Palindromic Paths in a Binary Tree

**Difficulty:** Medium
**Tags:** Bit Manipulation, Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given a binary tree where node values are digits from 1 to 9. A path in the binary tree is said to be <strong>pseudo-palindromic</strong> if at least one permutation of the node values in the path is a palindrome.</p>

<p><em>Return the number of <strong>pseudo-palindromic</strong> paths going from the root node to leaf nodes.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2020/05/06/palindromic_paths_1.png" style="width: 300px; height: 201px;" /></p>

<pre>
<strong>Input:</strong> root = [2,3,1,3,1,null,1]
<strong>Output:</strong> 2 
<strong>Explanation:</strong> The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the red path [2,3,3], the green path [2,1,1], and the path [2,3,1]. Among these paths only red path and green path are pseudo-palindromic paths since the red path [2,3,3] can be rearranged in [3,2,3] (palindrome) and the green path [2,1,1] can be rearranged in [1,2,1] (palindrome).
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/palindromic_paths_2.png" style="width: 300px; height: 314px;" /></strong></p>

<pre>
<strong>Input:</strong> root = [2,1,1,1,3,null,null,null,null,null,1]
<strong>Output:</strong> 1 
<strong>Explanation:</strong> The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the green path [2,1,1], the path [2,1,3,1], and the path [2,1]. Among these paths only the green path is pseudo-palindromic since [2,1,1] can be rearranged in [1,2,1] (palindrome).
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> root = [9]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 9</code></li>
</ul>


## Hints

1. Note that the node values of a path form a palindrome if at most one digit has an odd frequency (parity).
2. Use a Depth First Search (DFS) keeping the frequency (parity) of the digits. Once you are in a leaf node check if at most one digit has an odd frequency (parity).

## Solution

```rust
impl Solution { pub fn pseudo_palindromic_paths(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> i32 { fn black_dfs(node: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, mut black_mask: i32) -> i32 { if let Some(n) = node { let n = n.borrow(); black_mask ^= 1 << n.val; if n.left.is_none() && n.right.is_none() { return if (black_mask & (black_mask - 1)) == 0 { 1 } else { 0 }; } black_dfs(n.left.clone(), black_mask) + black_dfs(n.right.clone(), black_mask) } else { 0 } } black_dfs(root, 0) } }
```