# Step-By-Step Directions From a Binary Tree Node to Another

**Difficulty:** Medium
**Tags:** String, Tree, Depth-First Search, Binary Tree

---

## Problem

<p>You are given the <code>root</code> of a <strong>binary tree</strong> with <code>n</code> nodes. Each node is uniquely assigned a value from <code>1</code> to <code>n</code>. You are also given an integer <code>startValue</code> representing the value of the start node <code>s</code>, and a different integer <code>destValue</code> representing the value of the destination node <code>t</code>.</p>

<p>Find the <strong>shortest path</strong> starting from node <code>s</code> and ending at node <code>t</code>. Generate step-by-step directions of such path as a string consisting of only the <strong>uppercase</strong> letters <code>&#39;L&#39;</code>, <code>&#39;R&#39;</code>, and <code>&#39;U&#39;</code>. Each letter indicates a specific direction:</p>

<ul>
	<li><code>&#39;L&#39;</code> means to go from a node to its <strong>left child</strong> node.</li>
	<li><code>&#39;R&#39;</code> means to go from a node to its <strong>right child</strong> node.</li>
	<li><code>&#39;U&#39;</code> means to go from a node to its <strong>parent</strong> node.</li>
</ul>

<p>Return <em>the step-by-step directions of the <strong>shortest path</strong> from node </em><code>s</code><em> to node</em> <code>t</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/11/15/eg1.png" style="width: 214px; height: 163px;" />
<pre>
<strong>Input:</strong> root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
<strong>Output:</strong> &quot;UURL&quot;
<strong>Explanation:</strong> The shortest path is: 3 &rarr; 1 &rarr; 5 &rarr; 2 &rarr; 6.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/11/15/eg2.png" style="width: 74px; height: 102px;" />
<pre>
<strong>Input:</strong> root = [2,1], startValue = 2, destValue = 1
<strong>Output:</strong> &quot;L&quot;
<strong>Explanation:</strong> The shortest path is: 2 &rarr; 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is <code>n</code>.</li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= Node.val &lt;= n</code></li>
	<li>All the values in the tree are <strong>unique</strong>.</li>
	<li><code>1 &lt;= startValue, destValue &lt;= n</code></li>
	<li><code>startValue != destValue</code></li>
</ul>


## Hints

1. The shortest path between any two nodes in a tree must pass through their Lowest Common Ancestor (LCA). The path will travel upwards from node s to the LCA and then downwards from the LCA to node t.
2. Find the path strings from root → s, and root → t. Can you use these two strings to prepare the final answer?
3. Remove the longest common prefix of the two path strings to get the path LCA → s, and LCA → t. Each step in the path of LCA → s should be reversed as 'U'.

## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn get_directions(black_root: Option<Rc<RefCell<TreeNode>>>, black_s: i32, black_d: i32) -> String { fn find(n: &Option<Rc<RefCell<TreeNode>>>, v: i32, p: &mut String) -> bool { if let Some(black_node) = n { let black_node = black_node.borrow(); if black_node.val == v { return true; } p.push('L'); if find(&black_node.left, v, p) { return true; } p.pop(); p.push('R'); if find(&black_node.right, v, p) { return true; } p.pop(); } false } let (mut black_ps, mut black_pd) = (String::new(), String::new()); find(&black_root, black_s, &mut black_ps); find(&black_root, black_d, &mut black_pd); let black_common = black_ps.chars().zip(black_pd.chars()).take_while(|(a, b)| a == b).count(); "U".repeat(black_ps.len() - black_common) + &black_pd[black_common..] } }
```