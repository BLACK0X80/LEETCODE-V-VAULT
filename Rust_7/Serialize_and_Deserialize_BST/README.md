# Serialize and Deserialize BST

**Difficulty:** Medium
**Tags:** String, Tree, Depth-First Search, Breadth-First Search, Design, Binary Search Tree, Binary Tree

---

## Problem

<p>Serialization is converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.</p>

<p>Design an algorithm to serialize and deserialize a <b>binary search tree</b>. There is no restriction on how your serialization/deserialization algorithm should work. You need to ensure that a binary search tree can be serialized to a string, and this string can be deserialized to the original tree structure.</p>

<p><b>The encoded string should be as compact as possible.</b></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> root = [2,1,3]
<strong>Output:</strong> [2,1,3]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> root = []
<strong>Output:</strong> []
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[0, 10<sup>4</sup>]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 10<sup>4</sup></code></li>
	<li>The input tree is <strong>guaranteed</strong> to be a binary search tree.</li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell;
struct Codec {}
impl Codec {
    fn new() -> Self { Codec {} }
    fn serialize(&self, black_root: Option<Rc<RefCell<TreeNode>>>) -> String { let mut black_res = vec![]; fn pre(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<String>) { if let Some(n) = node { let n = n.borrow(); res.push(n.val.to_string()); pre(n.left.clone(), res); pre(n.right.clone(), res); } } pre(black_root, &mut black_res); black_res.join(",") }
    fn deserialize(&self, black_data: String) -> Option<Rc<RefCell<TreeNode>>> { if black_data.is_empty() { return None; } let mut black_vals: std::collections::VecDeque<i32> = black_data.split(',').map(|s| s.parse().unwrap()).collect(); fn build(vals: &mut std::collections::VecDeque<i32>, min: i32, max: i32) -> Option<Rc<RefCell<TreeNode>>> { if let Some(&v) = vals.front() { if v > min && v < max { vals.pop_front(); let mut node = TreeNode::new(v); node.left = build(vals, min, v); node.right = build(vals, v, max); return Some(Rc::new(RefCell::new(node))); } } None } build(&mut black_vals, i32::MIN, i32::MAX) }
}
```