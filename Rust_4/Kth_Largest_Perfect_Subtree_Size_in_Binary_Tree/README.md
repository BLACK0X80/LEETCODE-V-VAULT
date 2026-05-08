# K-th Largest Perfect Subtree Size in Binary Tree

**Difficulty:** Medium
**Tags:** Tree, Depth-First Search, Sorting, Binary Tree

---

## Problem

<p>You are given the <code>root</code> of a <strong>binary tree</strong> and an integer <code>k</code>.</p>

<p>Return an integer denoting the size of the <code>k<sup>th</sup></code> <strong>largest<em> </em>perfect binary</strong><em> </em><span data-keyword="subtree">subtree</span>, or <code>-1</code> if it doesn&#39;t exist.</p>

<p>A <strong>perfect binary tree</strong> is a tree where all leaves are on the same level, and every parent has two children.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">root = [5,3,6,5,2,5,7,1,8,null,null,6,8], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/10/14/tmpresl95rp-1.png" style="width: 400px; height: 173px;" /></p>

<p>The roots of the perfect binary subtrees are highlighted in black. Their sizes, in non-increasing order are <code>[3, 3, 1, 1, 1, 1, 1, 1]</code>.<br />
The <code>2<sup>nd</sup></code> largest size is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">root = [1,2,3,4,5,6,7], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/10/14/tmp_s508x9e-1.png" style="width: 300px; height: 189px;" /></p>

<p>The sizes of the perfect binary subtrees in non-increasing order are <code>[7, 3, 3, 1, 1, 1, 1]</code>. The size of the largest perfect binary subtree is 7.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">root = [1,2,3,null,4], k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">-1</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/10/14/tmp74xnmpj4-1.png" style="width: 250px; height: 225px;" /></p>

<p>The sizes of the perfect binary subtrees in non-increasing order are <code>[1, 1]</code>. There are fewer than 3 perfect binary subtrees.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 2000]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 2000</code></li>
	<li><code>1 &lt;= k &lt;= 1024</code></li>
</ul>


## Hints

1. For a subtree to form a perfect binary subtree, its children should also be perfect binary subtrees.
2. Check recursively that both the node and its children are perfect binary subtrees.
3. Gather all the perfect binary subtrees and return the kth largest.

## Solution

```rust
impl Solution { pub fn kth_largest_perfect_subtree(root: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, k: i32) -> i32 { let mut black_v = vec![]; fn dfs(n: Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>, v: &mut Vec<i32>) -> Option<(i32, i32)> { let n = n?; let (l, r) = (dfs(n.borrow().left.clone(), v), dfs(n.borrow().right.clone(), v)); match (l, r) { (None, None) if n.borrow().left.is_none() && n.borrow().right.is_none() => { v.push(1); Some((1, 1)) }, (Some((h1, s1)), Some((h2, s2))) if h1 == h2 => { v.push(s1 + s2 + 1); Some((h1 + 1, s1 + s2 + 1)) }, _ => None } } dfs(root, &mut black_v); black_v.sort_unstable_by(|a, b| b.cmp(a)); black_v.get(k as usize - 1).copied().unwrap_or(-1) } }
```