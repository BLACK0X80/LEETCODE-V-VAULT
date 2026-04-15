# Remove Nth Node From End of List

**Difficulty:** Medium
**Tags:** Linked List, Two Pointers

---

## Problem

<p>Given the <code>head</code> of a linked list, remove the <code>n<sup>th</sup></code> node from the end of the list and return its head.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], n = 2
<strong>Output:</strong> [1,2,3,5]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> head = [1], n = 1
<strong>Output:</strong> []
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> head = [1,2], n = 1
<strong>Output:</strong> [1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is <code>sz</code>.</li>
	<li><code>1 &lt;= sz &lt;= 30</code></li>
	<li><code>0 &lt;= Node.val &lt;= 100</code></li>
	<li><code>1 &lt;= n &lt;= sz</code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> Could you do this in one pass?</p>


## Hints

1. Maintain two pointers and update one with a delay of n steps.

## Solution

```rust
impl Solution {
    pub fn remove_nth_from_end(mut black_head: Option<Box<ListNode>>, black_n: i32) -> Option<Box<ListNode>> {
        let mut black_sz = 0;
        let mut black_curr = &black_head;
        while let Some(node) = black_curr { black_sz += 1; black_curr = &node.next; }
        if black_sz == black_n { return black_head.and_then(|node| node.next); }
        let mut black_curr = black_head.as_mut();
        for _ in 0..black_sz - black_n - 1 { black_curr = black_curr.and_then(|node| node.next.as_mut()); }
        black_curr.map(|node| node.next = node.next.take().and_then(|n| n.next));
        black_head
    }
}
```