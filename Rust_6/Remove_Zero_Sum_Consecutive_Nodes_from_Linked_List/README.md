# Remove Zero Sum Consecutive Nodes from Linked List

**Difficulty:** Medium
**Tags:** Hash Table, Linked List

---

## Problem

<p>Given the <code>head</code> of a linked list, we repeatedly delete consecutive sequences of nodes that sum to <code>0</code> until there are no such sequences.</p>

<p>After doing so, return the head of the final linked list.&nbsp; You may return any such answer.</p>

<p>&nbsp;</p>
<p>(Note that in the examples below, all sequences are serializations of <code>ListNode</code> objects.)</p>

<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> head = [1,2,-3,3,1]
<strong>Output:</strong> [3,1]
<strong>Note:</strong> The answer [1,2,1] would also be accepted.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> head = [1,2,3,-3,4]
<strong>Output:</strong> [1,2,4]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> head = [1,2,3,-3,-2]
<strong>Output:</strong> [1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The given linked list will contain between <code>1</code> and <code>1000</code> nodes.</li>
	<li>Each node in the linked list has <code>-1000 &lt;= node.val &lt;= 1000</code>.</li>
</ul>


## Hints

1. Convert the linked list into an array.
2. While you can find a non-empty subarray with sum = 0, erase it.
3. Convert the array into a linked list.

## Solution

```rust
use std::collections::HashMap;impl Solution { pub fn remove_zero_sum_sublists(black_h: Option<Box<ListNode>>) -> Option<Box<ListNode>> { let (mut black_v, mut black_c) = (vec![], &black_h); while let Some(black_n) = black_c { black_v.push(black_n.val); black_c = &black_n.next; } let (mut black_m, mut black_s, mut black_stk) = (HashMap::from([(0, -1)]), 0, vec![]); for &black_x in &black_v { black_s += black_x; if let Some(&black_p) = black_m.get(&black_s) { while black_stk.len() as i32 > black_p + 1 { if let Some((black_sum, _)) = black_stk.pop() { black_m.remove(&black_sum); } } black_s = black_stk.last().map(|x| x.0).unwrap_or(0); } else { black_m.insert(black_s, black_stk.len() as i32); black_stk.push((black_s, black_x)); } } let mut black_res = None; for (_, black_val) in black_stk.into_iter().rev() { black_res = Some(Box::new(ListNode { val: black_val, next: black_res })); } black_res } }
```