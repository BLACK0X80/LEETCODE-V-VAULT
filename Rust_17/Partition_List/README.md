# Partition List

**Difficulty:** Medium
**Tags:** Linked List, Two Pointers

---

## Problem

<p>Given the <code>head</code> of a linked list and a value <code>x</code>, partition it such that all nodes <strong>less than</strong> <code>x</code> come before nodes <strong>greater than or equal</strong> to <code>x</code>.</p>

<p>You should <strong>preserve</strong> the original relative order of the nodes in each of the two partitions.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/partition.jpg" style="width: 662px; height: 222px;" />
<pre>
<strong>Input:</strong> head = [1,4,3,2,5,2], x = 3
<strong>Output:</strong> [1,2,2,4,3,5]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> head = [2,1], x = 2
<strong>Output:</strong> [1,2]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is in the range <code>[0, 200]</code>.</li>
	<li><code>-100 &lt;= Node.val &lt;= 100</code></li>
	<li><code>-200 &lt;= x &lt;= 200</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn partition(mut black_h: Option<Box<ListNode>>, black_x: i32) -> Option<Box<ListNode>> {
        let (mut black_l1, mut black_l2) = (Box::new(ListNode::new(0)), Box::new(ListNode::new(0)));
        let (mut black_p1, mut black_p2) = (&mut black_l1, &mut black_l2);
        while let Some(mut black_node) = black_h {
            black_h = black_node.next.take();
            if black_node.val < black_x { black_p1.next = Some(black_node); black_p1 = black_p1.next.as_mut().unwrap(); }
            else { black_p2.next = Some(black_node); black_p2 = black_p2.next.as_mut().unwrap(); }
        }
        black_p1.next = black_l2.next;
        black_l1.next
    }
}
```