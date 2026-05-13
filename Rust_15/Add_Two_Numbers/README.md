# Add Two Numbers

**Difficulty:** Medium
**Tags:** Linked List, Math, Recursion

---

## Problem

<p>You are given two <strong>non-empty</strong> linked lists representing two non-negative integers. The digits are stored in <strong>reverse order</strong>, and each of their nodes contains a single digit. Add the two numbers and return the sum&nbsp;as a linked list.</p>

<p>You may assume the two numbers do not contain any leading zero, except the number 0 itself.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
<pre>
<strong>Input:</strong> l1 = [2,4,3], l2 = [5,6,4]
<strong>Output:</strong> [7,0,8]
<strong>Explanation:</strong> 342 + 465 = 807.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> l1 = [0], l2 = [0]
<strong>Output:</strong> [0]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
<strong>Output:</strong> [8,9,9,9,0,0,0,1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in each linked list is in the range <code>[1, 100]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 9</code></li>
	<li>It is guaranteed that the list represents a number that does not have leading zeros.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut black_head = Some(Box::new(ListNode::new(0)));
        let mut black_ptr = &mut black_head;
        let (mut black_l1, mut black_l2, mut black_carry) = (l1, l2, 0);

        while black_l1.is_some() || black_l2.is_some() || black_carry != 0 {
            let mut black_sum = black_carry;
            
            if let Some(black_node) = black_l1 {
                black_sum += black_node.val;
                black_l1 = black_node.next;
            }
            
            if let Some(black_node) = black_l2 {
                black_sum += black_node.val;
                black_l2 = black_node.next;
            }

            black_carry = black_sum / 10;
            
            if let Some(black_curr) = black_ptr {
                black_curr.next = Some(Box::new(ListNode::new(black_sum % 10)));
                black_ptr = &mut black_curr.next;
            }
        }

        black_head.unwrap().next
    }
}
```