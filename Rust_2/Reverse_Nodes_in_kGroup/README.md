# Reverse Nodes in k-Group

**Difficulty:** Hard
**Tags:** Linked List, Recursion

---

## Problem

<p>Given the <code>head</code> of a linked list, reverse the nodes of the list <code>k</code> at a time, and return <em>the modified list</em>.</p>

<p><code>k</code> is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of <code>k</code> then left-out nodes, in the end, should remain as it is.</p>

<p>You may not alter the values in the list&#39;s nodes, only nodes themselves may be changed.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg" style="width: 542px; height: 222px;" />
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], k = 2
<strong>Output:</strong> [2,1,4,3,5]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg" style="width: 542px; height: 222px;" />
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], k = 3
<strong>Output:</strong> [3,2,1,4,5]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is <code>n</code>.</li>
	<li><code>1 &lt;= k &lt;= n &lt;= 5000</code></li>
	<li><code>0 &lt;= Node.val &lt;= 1000</code></li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow-up:</strong> Can you solve the problem in <code>O(1)</code> extra memory space?</p>



## Solution

```rust
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut nodes = Vec::new();
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next.take();
            nodes.push(node);
        }

        let n = nodes.len();
        let k = k as usize;

        for chunk in nodes.chunks_mut(k) {
            if chunk.len() == k {
                chunk.reverse();
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        for mut node in nodes {
            node.next = None;
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
```