# Most Frequent Subtree Sum

**Difficulty:** Medium
**Tags:** Hash Table, Tree, Depth-First Search, Binary Tree

---

## Problem

<p>Given the <code>root</code> of a binary tree, return the most frequent <strong>subtree sum</strong>. If there is a tie, return all the values with the highest frequency in any order.</p>

<p>The <strong>subtree sum</strong> of a node is defined as the sum of all the node values formed by the subtree rooted at that node (including the node itself).</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/freq1-tree.jpg" style="width: 207px; height: 183px;" />
<pre>
<strong>Input:</strong> root = [5,2,-3]
<strong>Output:</strong> [2,-3,4]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/freq2-tree.jpg" style="width: 207px; height: 183px;" />
<pre>
<strong>Input:</strong> root = [5,2,-5]
<strong>Output:</strong> [2]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>4</sup>]</code>.</li>
	<li><code>-10<sup>5</sup> &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>



## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; use std::collections::HashMap; impl Solution { pub fn find_frequent_tree_sum(black_root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> { let mut black_cnt = HashMap::new(); fn dfs(n: Option<Rc<RefCell<TreeNode>>>, cnt: &mut HashMap<i32, i32>) -> i32 { match n { Some(node) => { let node = node.borrow(); let s = node.val + dfs(node.left.clone(), cnt) + dfs(node.right.clone(), cnt); *cnt.entry(s).or_insert(0) += 1; s } None => 0 } } dfs(black_root, &mut black_cnt); let black_max = *black_cnt.values().max().unwrap_or(&0); black_cnt.into_iter().filter(|&(_, v)| v == black_max).map(|(k, _)| k).collect() } }
```