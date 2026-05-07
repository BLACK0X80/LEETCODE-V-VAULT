# Binary Search Tree Iterator

**Difficulty:** Medium
**Tags:** Stack, Tree, Design, Binary Search Tree, Binary Tree, Iterator

---

## Problem

<p>Implement the <code>BSTIterator</code> class that represents an iterator over the <strong><a href="https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR)" target="_blank">in-order traversal</a></strong> of a binary search tree (BST):</p>

<ul>
	<li><code>BSTIterator(TreeNode root)</code> Initializes an object of the <code>BSTIterator</code> class. The <code>root</code> of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.</li>
	<li><code>boolean hasNext()</code> Returns <code>true</code> if there exists a number in the traversal to the right of the pointer, otherwise returns <code>false</code>.</li>
	<li><code>int next()</code> Moves the pointer to the right, then returns the number at the pointer.</li>
</ul>

<p>Notice that by initializing the pointer to a non-existent smallest number, the first call to <code>next()</code> will return the smallest element in the BST.</p>

<p>You may assume that <code>next()</code> calls will always be valid. That is, there will be at least a next number in the in-order traversal when <code>next()</code> is called.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2018/12/25/bst-tree.png" style="width: 189px; height: 178px;" />
<pre>
<strong>Input</strong>
[&quot;BSTIterator&quot;, &quot;next&quot;, &quot;next&quot;, &quot;hasNext&quot;, &quot;next&quot;, &quot;hasNext&quot;, &quot;next&quot;, &quot;hasNext&quot;, &quot;next&quot;, &quot;hasNext&quot;]
[[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
<strong>Output</strong>
[null, 3, 7, true, 9, true, 15, true, 20, false]

<strong>Explanation</strong>
BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
bSTIterator.next();    // return 3
bSTIterator.next();    // return 7
bSTIterator.hasNext(); // return True
bSTIterator.next();    // return 9
bSTIterator.hasNext(); // return True
bSTIterator.next();    // return 15
bSTIterator.hasNext(); // return True
bSTIterator.next();    // return 20
bSTIterator.hasNext(); // return False
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 10<sup>6</sup></code></li>
	<li>At most <code>10<sup>5</sup></code> calls will be made to <code>hasNext</code>, and <code>next</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong></p>

<ul>
	<li>Could you implement <code>next()</code> and <code>hasNext()</code> to run in average <code>O(1)</code> time and use&nbsp;<code>O(h)</code> memory, where <code>h</code> is the height of the tree?</li>
</ul>



## Solution

```rust
use std::{rc::Rc, cell::RefCell};
struct BSTIterator { black_v: Vec<i32>, black_i: usize }
impl BSTIterator {
    fn new(black_r: Option<Rc<RefCell<TreeNode>>>) -> Self { let mut black_v = vec![]; Self::black_h(black_r, &mut black_v); Self { black_v, black_i: 0 } }
    fn black_h(black_n: Option<Rc<RefCell<TreeNode>>>, black_v: &mut Vec<i32>) { if let Some(black_node) = black_n { let black_b = black_node.borrow(); Self::black_h(black_b.left.clone(), black_v); black_v.push(black_b.val); Self::black_h(black_b.right.clone(), black_v); } }
    fn next(&mut self) -> i32 { let black_val = self.black_v[self.black_i]; self.black_i += 1; black_val }
    fn has_next(&self) -> bool { self.black_i < self.black_v.len() }
}
```