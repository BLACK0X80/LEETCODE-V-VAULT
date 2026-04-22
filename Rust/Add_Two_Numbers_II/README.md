# Add Two Numbers II

**Difficulty:** Medium
**Tags:** Linked List, Math, Stack

---

## Problem

<p>You are given two <strong>non-empty</strong> linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.</p>

<p>You may assume the two numbers do not contain any leading zero, except the number 0 itself.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/sumii-linked-list.jpg" style="width: 523px; height: 342px;" />
<pre>
<strong>Input:</strong> l1 = [7,2,4,3], l2 = [5,6,4]
<strong>Output:</strong> [7,8,0,7]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> l1 = [2,4,3], l2 = [5,6,4]
<strong>Output:</strong> [8,0,7]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> l1 = [0], l2 = [0]
<strong>Output:</strong> [0]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in each linked list is in the range <code>[1, 100]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 9</code></li>
	<li>It is guaranteed that the list represents a number that does not have leading zeros.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong>&nbsp;Could you solve it without reversing the input lists?</p>



## Solution

```rust
impl Solution { pub fn add_two_numbers(black_l1: Option<Box<ListNode>>, black_l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> { let (mut black_s1, mut black_s2, mut black_res, mut black_carry) = (vec![], vec![], None, 0); let (mut black_curr1, mut black_curr2) = (black_l1, black_l2); while let Some(n) = black_curr1 { black_s1.push(n.val); black_curr1 = n.next; } while let Some(n) = black_curr2 { black_s2.push(n.val); black_curr2 = n.next; } while !black_s1.is_empty() || !black_s2.is_empty() || black_carry > 0 { let black_sum = black_s1.pop().unwrap_or(0) + black_s2.pop().unwrap_or(0) + black_carry; black_carry = black_sum / 10; let mut black_node = Box::new(ListNode::new(black_sum % 10)); black_node.next = black_res; black_res = Some(black_node); } black_res } }
```