# Find Elements in a Contaminated Binary Tree

**Difficulty:** Medium
**Tags:** Hash Table, Tree, Depth-First Search, Breadth-First Search, Design, Binary Tree

---

## Problem

<p>Given a binary tree with the following rules:</p>

<ol>
	<li><code>root.val == 0</code></li>
	<li>For any <code>treeNode</code>:
	<ol type="a">
		<li>If <code>treeNode.val</code> has a value <code>x</code> and <code>treeNode.left != null</code>, then <code>treeNode.left.val == 2 * x + 1</code></li>
		<li>If <code>treeNode.val</code> has a value <code>x</code> and <code>treeNode.right != null</code>, then <code>treeNode.right.val == 2 * x + 2</code></li>
	</ol>
	</li>
</ol>

<p>Now the binary tree is contaminated, which means all <code>treeNode.val</code> have been changed to <code>-1</code>.</p>

<p>Implement the <code>FindElements</code> class:</p>

<ul>
	<li><code>FindElements(TreeNode* root)</code> Initializes the object with a contaminated binary tree and recovers it.</li>
	<li><code>bool find(int target)</code> Returns <code>true</code> if the <code>target</code> value exists in the recovered binary tree.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4-1.jpg" style="width: 320px; height: 119px;" />
<pre>
<strong>Input</strong>
[&quot;FindElements&quot;,&quot;find&quot;,&quot;find&quot;]
[[[-1,null,-1]],[1],[2]]
<strong>Output</strong>
[null,false,true]
<strong>Explanation</strong>
FindElements findElements = new FindElements([-1,null,-1]); 
findElements.find(1); // return False 
findElements.find(2); // return True </pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4.jpg" style="width: 400px; height: 198px;" />
<pre>
<strong>Input</strong>
[&quot;FindElements&quot;,&quot;find&quot;,&quot;find&quot;,&quot;find&quot;]
[[[-1,-1,-1,-1,-1]],[1],[3],[5]]
<strong>Output</strong>
[null,true,true,false]
<strong>Explanation</strong>
FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
findElements.find(1); // return True
findElements.find(3); // return True
findElements.find(5); // return False</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/11/07/untitled-diagram-4-1-1.jpg" style="width: 306px; height: 274px;" />
<pre>
<strong>Input</strong>
[&quot;FindElements&quot;,&quot;find&quot;,&quot;find&quot;,&quot;find&quot;,&quot;find&quot;]
[[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
<strong>Output</strong>
[null,true,false,false,true]
<strong>Explanation</strong>
FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
findElements.find(2); // return True
findElements.find(3); // return False
findElements.find(4); // return False
findElements.find(5); // return True
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>TreeNode.val == -1</code></li>
	<li>The height of the binary tree is less than or equal to <code>20</code></li>
	<li>The total number of nodes is between <code>[1, 10<sup>4</sup>]</code></li>
	<li>Total calls of <code>find()</code> is between <code>[1, 10<sup>4</sup>]</code></li>
	<li><code>0 &lt;= target &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Use DFS to traverse the binary tree and recover it.
2. Use a hashset to store TreeNode.val for finding.

## Solution

```rust
use std::{collections::HashSet, rc::Rc, cell::RefCell};
struct FindElements { black_s: HashSet<i32> }
impl FindElements { fn new(black_r: Option<Rc<RefCell<TreeNode>>>) -> Self { let mut black_s = HashSet::new(); fn black_f(black_n: Option<Rc<RefCell<TreeNode>>>, black_v: i32, black_set: &mut HashSet<i32>) { if let Some(black_node) = black_n { black_set.insert(black_v); black_f(black_node.borrow().left.clone(), 2 * black_v + 1, black_set); black_f(black_node.borrow().right.clone(), 2 * black_v + 2, black_set); } } black_f(black_r, 0, &mut black_s); Self { black_s } } fn find(&self, black_t: i32) -> bool { self.black_s.contains(&black_t) } }
```