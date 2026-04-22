# Linked List Components

**Difficulty:** Medium
**Tags:** Array, Hash Table, Linked List

---

## Problem

<p>You are given the <code>head</code> of a linked list containing unique integer values and an integer array <code>nums</code> that is a subset of the linked list values.</p>

<p>Return <em>the number of connected components in </em><code>nums</code><em> where two values are connected if they appear <strong>consecutively</strong> in the linked list</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/07/22/lc-linkedlistcom1.jpg" style="width: 424px; height: 65px;" />
<pre>
<strong>Input:</strong> head = [0,1,2,3], nums = [0,1,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 0 and 1 are connected, so [0, 1] and [3] are the two connected components.
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/07/22/lc-linkedlistcom2.jpg" style="width: 544px; height: 65px;" />
<pre>
<strong>Input:</strong> head = [0,1,2,3,4], nums = [0,3,1,4]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 0 and 1 are connected, 3 and 4 are connected, so [0, 1] and [3, 4] are the two connected components.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the linked list is <code>n</code>.</li>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= Node.val &lt; n</code></li>
	<li>All the values <code>Node.val</code> are <strong>unique</strong>.</li>
	<li><code>1 &lt;= nums.length &lt;= n</code></li>
	<li><code>0 &lt;= nums[i] &lt; n</code></li>
	<li>All the values of <code>nums</code> are <strong>unique</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn num_components(black_head: Option<Box<ListNode>>, black_nums: Vec<i32>) -> i32 { let black_set: std::collections::HashSet<_> = black_nums.into_iter().collect(); let (mut black_ans, mut black_in_comp, mut black_curr) = (0, false, &black_head); while let Some(node) = black_curr { if black_set.contains(&node.val) { if !black_in_comp { black_ans += 1; black_in_comp = true; } } else { black_in_comp = false; } black_curr = &node.next; } black_ans } }
```