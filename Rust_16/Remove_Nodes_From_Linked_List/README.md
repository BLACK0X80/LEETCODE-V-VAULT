# Remove Nodes From Linked List

**Difficulty:** Medium
**Tags:** Linked List, Stack, Recursion, Monotonic Stack

---

## Problem

<p>You are given the <code>head</code> of a linked list.</p>

<p>Remove every node which has a node with a greater value anywhere to the right side of it.</p>

<p>Return <em>the </em><code>head</code><em> of the modified linked list.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2022/10/02/drawio.png" style="width: 631px; height: 51px;" />
<pre>
<strong>Input:</strong> head = [5,2,13,3,8]
<strong>Output:</strong> [13,8]
<strong>Explanation:</strong> The nodes that should be removed are 5, 2 and 3.
- Node 13 is to the right of node 5.
- Node 13 is to the right of node 2.
- Node 8 is to the right of node 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> head = [1,1,1,1]
<strong>Output:</strong> [1,1,1,1]
<strong>Explanation:</strong> Every node has value 1, so no nodes are removed.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of the nodes in the given list is in the range <code>[1, 10<sup>5</sup>]</code>.</li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Iterate on nodes in reversed order.
2. When iterating in reversed order, save the maximum value that was passed before.

## Solution

```rust
impl Solution { pub fn remove_nodes(black_h: Option<Box<ListNode>>) -> Option<Box<ListNode>> { let (mut black_v, mut black_c) = (vec![], black_h); while let Some(black_node) = black_c { black_v.push(black_node.val); black_c = black_node.next; } let mut black_stk: Vec<i32> = vec![]; for black_x in black_v { while let Some(&black_t) = black_stk.last() { if black_t < black_x { black_stk.pop(); } else { break; } } black_stk.push(black_x); } let mut black_res = None; for &black_x in black_stk.iter().rev() { black_res = Some(Box::new(ListNode { val: black_x, next: black_res })); } black_res } }
```