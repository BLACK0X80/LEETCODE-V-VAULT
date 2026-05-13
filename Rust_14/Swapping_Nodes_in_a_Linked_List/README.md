# Swapping Nodes in a Linked List

**Difficulty:** Medium
**Tags:** Linked List, Two Pointers

---

## Problem

<p>You are given the <code>head</code> of a linked list, and an integer <code>k</code>.</p>

<p>Return <em>the head of the linked list after <strong>swapping</strong> the values of the </em><code>k<sup>th</sup></code> <em>node from the beginning and the </em><code>k<sup>th</sup></code> <em>node from the end (the list is <strong>1-indexed</strong>).</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/linked1.jpg" style="width: 400px; height: 112px;" />
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], k = 2
<strong>Output:</strong> [1,4,3,2,5]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> head = [7,9,6,6,7,8,3,0,9,5], k = 5
<strong>Output:</strong> [7,9,6,6,8,7,3,0,9,5]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is <code>n</code>.</li>
	<li><code>1 &lt;= k &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= Node.val &lt;= 100</code></li>
</ul>


## Hints

1. We can traverse the linked list and store the elements in an array.
2. Upon conversion to an array, we can swap the required elements by indexing the array.
3. We can rebuild the linked list using the order of the elements in the array.

## Solution

```rust
impl Solution { pub fn swap_nodes(mut black_head: Option<Box<ListNode>>, black_k: i32) -> Option<Box<ListNode>> { let mut black_v = vec![]; let mut black_curr = &black_head; while let Some(black_node) = black_curr { black_v.push(black_node.val); black_curr = &black_node.next; } let black_n = black_v.len(); black_v.swap(black_k as usize - 1, black_n - black_k as usize); let (mut black_curr, mut black_i) = (&mut black_head, 0); while let Some(black_node) = black_curr { black_node.val = black_v[black_i]; black_i += 1; black_curr = &mut black_node.next; } black_head } }
```