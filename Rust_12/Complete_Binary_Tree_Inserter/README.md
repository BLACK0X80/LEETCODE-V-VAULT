# Complete Binary Tree Inserter

**Difficulty:** Medium
**Tags:** Tree, Breadth-First Search, Design, Binary Tree

---

## Problem

<p>A <strong>complete binary tree</strong> is a binary tree in which every level, except possibly the last, is completely filled, and all nodes are as far left as possible.</p>

<p>Design an algorithm to insert a new node to a complete binary tree keeping it complete after the insertion.</p>

<p>Implement the <code>CBTInserter</code> class:</p>

<ul>
	<li><code>CBTInserter(TreeNode root)</code> Initializes the data structure with the <code>root</code> of the complete binary tree.</li>
	<li><code>int insert(int v)</code> Inserts a <code>TreeNode</code> into the tree with value <code>Node.val == val</code> so that the tree remains complete, and returns the value of the parent of the inserted <code>TreeNode</code>.</li>
	<li><code>TreeNode get_root()</code> Returns the root node of the tree.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/lc-treeinsert.jpg" style="width: 500px; height: 143px;" />
<pre>
<strong>Input</strong>
[&quot;CBTInserter&quot;, &quot;insert&quot;, &quot;insert&quot;, &quot;get_root&quot;]
[[[1, 2]], [3], [4], []]
<strong>Output</strong>
[null, 1, 2, [1, 2, 3, 4]]

<strong>Explanation</strong>
CBTInserter cBTInserter = new CBTInserter([1, 2]);
cBTInserter.insert(3);  // return 1
cBTInserter.insert(4);  // return 2
cBTInserter.get_root(); // return [1, 2, 3, 4]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree will be in the range <code>[1, 1000]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 5000</code></li>
	<li><code>root</code> is a complete binary tree.</li>
	<li><code>0 &lt;= val &lt;= 5000</code></li>
	<li>At most <code>10<sup>4</sup></code> calls will be made to <code>insert</code> and <code>get_root</code>.</li>
</ul>



## Solution

```rust
struct CBTInserter { black_nodes: Vec<Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>> } impl CBTInserter { fn new(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>) -> Self { let (mut black_list, mut black_q) = (vec![], std::collections::VecDeque::from([root])); while let Some(Some(black_n)) = black_q.pop_front() { black_list.push(Some(black_n.clone())); black_q.push_back(black_n.borrow().left.clone()); black_q.push_back(black_n.borrow().right.clone()); } Self { black_nodes: black_list } } fn insert(&mut self, v: i32) -> i32 { let (black_n, black_new) = (self.black_nodes.len(), Some(std::rc::Rc::new(std::cell::RefCell::new(TreeNode::new(v))))); let black_p = (black_n - 1) / 2; if black_n % 2 == 1 { self.black_nodes[black_p].as_ref().unwrap().borrow_mut().left = black_new.clone(); } else { self.black_nodes[black_p].as_ref().unwrap().borrow_mut().right = black_new.clone(); } self.black_nodes.push(black_new); self.black_nodes[black_p].as_ref().unwrap().borrow().val } fn get_root(&self) -> Option<std::rc::Rc<std::cell::RefCell<TreeNode>>> { self.black_nodes[0].clone() } }
```