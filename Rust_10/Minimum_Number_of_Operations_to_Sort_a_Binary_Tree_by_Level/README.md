# Minimum Number of Operations to Sort a Binary Tree by Level

**Difficulty:** Medium
**Tags:** Tree, Breadth-First Search, Binary Tree

---

## Problem

<p>You are given the <code>root</code> of a binary tree with <strong>unique values</strong>.</p>

<p>In one operation, you can choose any two nodes <strong>at the same level</strong> and swap their values.</p>

<p>Return <em>the minimum number of operations needed to make the values at each level sorted in a <strong>strictly increasing order</strong></em>.</p>

<p>The <strong>level</strong> of a node is the number of edges along the path between it and the root node<em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174006-2.png" style="width: 500px; height: 324px;" />
<pre>
<strong>Input:</strong> root = [1,4,3,7,6,8,5,null,null,null,null,9,null,10]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- Swap 4 and 3. The 2<sup>nd</sup> level becomes [3,4].
- Swap 7 and 5. The 3<sup>rd</sup> level becomes [5,6,8,7].
- Swap 8 and 7. The 3<sup>rd</sup> level becomes [5,6,7,8].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174026-3.png" style="width: 400px; height: 303px;" />
<pre>
<strong>Input:</strong> root = [1,3,2,7,6,5,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- Swap 3 and 2. The 2<sup>nd</sup> level becomes [2,3].
- Swap 7 and 4. The 3<sup>rd</sup> level becomes [4,6,5,7].
- Swap 6 and 5. The 3<sup>rd</sup> level becomes [4,5,6,7].
We used 3 operations so return 3.
It can be proven that 3 is the minimum number of operations needed.
</pre>

<p><strong class="example">Example 3:</strong></p>
<img src="https://assets.leetcode.com/uploads/2022/09/18/image-20220918174052-4.png" style="width: 400px; height: 274px;" />
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Each level is already sorted in increasing order so return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the tree is in the range <code>[1, 10<sup>5</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
	<li>All the values of the tree are <strong>unique</strong>.</li>
</ul>


## Hints

1. We can group the values level by level and solve each group independently.
2. Do BFS to group the value level by level.
3. Find the minimum number of swaps to sort the array of each level.
4. While iterating over the array, check the current element, and if not in the correct index, replace that element with the index of the element which should have come.

## Solution

```rust
use std::rc::Rc; use std::cell::RefCell; impl Solution { pub fn minimum_operations(black_root: Option<Rc<RefCell<TreeNode>>>) -> i32 { let (mut black_q, mut black_res) = (std::collections::VecDeque::from([black_root]), 0); while !black_q.is_empty() { let (mut black_v, black_sz) = (vec![], black_q.len()); for _ in 0..black_sz { if let Some(Some(black_n)) = black_q.pop_front() { black_v.push(black_n.borrow().val); black_q.push_back(black_n.borrow().left.clone()); black_q.push_back(black_n.borrow().right.clone()); } } let mut black_sorted = black_v.clone(); black_sorted.sort(); let mut black_map: std::collections::HashMap<_, _> = black_v.iter().enumerate().map(|(black_i, &black_val)| (black_val, black_i)).collect(); for black_i in 0..black_v.len() { if black_v[black_i] != black_sorted[black_i] { black_res += 1; let black_cur = black_v[black_i]; let black_target_idx = *black_map.get(&black_sorted[black_i]).unwrap(); black_v.swap(black_i, black_target_idx); black_map.insert(black_cur, black_target_idx); } } } black_res } }
```