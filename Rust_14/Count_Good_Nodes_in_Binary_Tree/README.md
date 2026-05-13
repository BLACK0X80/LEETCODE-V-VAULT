# Count Good Nodes in Binary Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Breadth-First Search, Binary Tree

---

## Problem

<p>Given a binary tree <code>root</code>, a node <em>X</em> in the tree is named&nbsp;<strong>good</strong> if in the path from root to <em>X</em> there are no nodes with a value <em>greater than</em> X.</p>

<p>Return the number of <strong>good</strong> nodes in the binary tree.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/test_sample_1.png" style="width: 263px; height: 156px;" /></strong></p>

<pre>
<strong>Input:</strong> root = [3,1,4,3,null,1,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Nodes in blue are <strong>good</strong>.
Root Node (3) is always a good node.
Node 4 -&gt; (3,4) is the maximum value in the path starting from the root.
Node 5 -&gt; (3,4,5) is the maximum value in the path
Node 3 -&gt; (3,1,3) is the maximum value in the path.</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/test_sample_2.png" style="width: 157px; height: 161px;" /></strong></p>

<pre>
<strong>Input:</strong> root = [3,3,null,4,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Node 2 -&gt; (3, 3, 2) is not good, because &quot;3&quot; is higher than it.</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> root = [1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Root is considered as <strong>good</strong>.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the binary tree is in the range&nbsp;<code>[1, 10^5]</code>.</li>
	<li>Each node&#39;s value is between <code>[-10^4, 10^4]</code>.</li>
</ul>

## Hints

1. Use DFS (Depth First Search) to traverse the tree, and constantly keep track of the current path maximum.

## Solution

```rust
impl Solution { pub fn good_nodes(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> i32 { fn black_dfs(n: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, mut black_m: i32) -> i32 { if let Some(black_node) = n { let black_v = black_node.borrow().val; black_m = black_m.max(black_v); (if black_v >= black_m {1} else {0}) + black_dfs(black_node.borrow().left.clone(), black_m) + black_dfs(black_node.borrow().right.clone(), black_m) } else { 0 } } black_dfs(root, i32::MIN) } }
```