# Next Greater Node In Linked List

**Difficulty:** Medium
**Tags:** Array, Linked List, Stack, Monotonic Stack

---

## Problem

<p>You are given the <code>head</code> of a linked list with <code>n</code> nodes.</p>

<p>For each node in the list, find the value of the <strong>next greater node</strong>. That is, for each node, find the value of the first node that is next to it and has a <strong>strictly larger</strong> value than it.</p>

<p>Return an integer array <code>answer</code> where <code>answer[i]</code> is the value of the next greater node of the <code>i<sup>th</sup></code> node (<strong>1-indexed</strong>). If the <code>i<sup>th</sup></code> node does not have a next greater node, set <code>answer[i] = 0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/05/linkedlistnext1.jpg" style="width: 304px; height: 133px;" />
<pre>
<strong>Input:</strong> head = [2,1,5]
<strong>Output:</strong> [5,5,0]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/08/05/linkedlistnext2.jpg" style="width: 500px; height: 113px;" />
<pre>
<strong>Input:</strong> head = [2,7,4,3,5]
<strong>Output:</strong> [7,0,5,5,0]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is <code>n</code>.</li>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>1 &lt;= Node.val &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. We can use a stack that stores nodes in monotone decreasing order of value.  When we see a node_j with a larger value, every node_i in the stack has next_larger(node_i) = node_j .

## Solution

```rust
impl Solution { pub fn next_larger_nodes(black_h: Option<Box<ListNode>>) -> Vec<i32> { let (mut black_v, mut black_curr) = (vec![], &black_h); while let Some(black_node) = black_curr { black_v.push(black_node.val); black_curr = &black_node.next; } let (mut black_res, mut black_stk) = (vec![0; black_v.len()], vec![]); for black_i in 0..black_v.len() { while !black_stk.is_empty() && black_v[*black_stk.last().unwrap()] < black_v[black_i] { black_res[black_stk.pop().unwrap()] = black_v[black_i]; } black_stk.push(black_i); } black_res } }
```