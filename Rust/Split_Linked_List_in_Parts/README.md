# Split Linked List in Parts

**Difficulty:** Medium
**Tags:** Linked List

---

## Problem

<p>Given the <code>head</code> of a singly linked list and an integer <code>k</code>, split the linked list into <code>k</code> consecutive linked list parts.</p>

<p>The length of each part should be as equal as possible: no two parts should have a size differing by more than one. This may lead to some parts being null.</p>

<p>The parts should be in the order of occurrence in the input list, and parts occurring earlier should always have a size greater than or equal to parts occurring later.</p>

<p>Return <em>an array of the </em><code>k</code><em> parts</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/split1-lc.jpg" style="width: 400px; height: 134px;" />
<pre>
<strong>Input:</strong> head = [1,2,3], k = 5
<strong>Output:</strong> [[1],[2],[3],[],[]]
<strong>Explanation:</strong>
The first element output[0] has output[0].val = 1, output[0].next = null.
The last element output[4] is null, but its string representation as a ListNode is [].
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/split2-lc.jpg" style="width: 600px; height: 60px;" />
<pre>
<strong>Input:</strong> head = [1,2,3,4,5,6,7,8,9,10], k = 3
<strong>Output:</strong> [[1,2,3,4],[5,6,7],[8,9,10]]
<strong>Explanation:</strong>
The input has been split into consecutive parts with size difference at most 1, and earlier parts are a larger size than the later parts.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is in the range <code>[0, 1000]</code>.</li>
	<li><code>0 &lt;= Node.val &lt;= 1000</code></li>
	<li><code>1 &lt;= k &lt;= 50</code></li>
</ul>


## Hints

1. If there are N nodes in the list, and k parts, then every part has N/k elements, except the first N%k parts have an extra one.

## Solution

```rust
impl Solution { pub fn split_list_to_parts(black_head: Option<Box<ListNode>>, black_k: i32) -> Vec<Option<Box<ListNode>>> { let (mut black_len, mut black_curr) = (0, &black_head); while let Some(node) = black_curr { black_len += 1; black_curr = &node.next; } let (black_size, mut black_rem) = (black_len / black_k, black_len % black_k); let (mut black_res, mut black_head) = (vec![], black_head); for _ in 0..black_k { let mut black_part_head = black_head; let mut black_curr = &mut black_part_head; let black_curr_part_size = black_size + if black_rem > 0 { 1 } else { 0 }; if black_rem > 0 { black_rem -= 1; } for _ in 0..black_curr_part_size - 1 { if let Some(node) = black_curr { black_curr = &mut node.next; } } if let Some(node) = black_curr { black_head = node.next.take(); } else { black_head = None; } black_res.push(black_part_head); } black_res } }
```