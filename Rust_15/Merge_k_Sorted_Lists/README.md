# Merge k Sorted Lists

**Difficulty:** Hard
**Tags:** Linked List, Divide and Conquer, Heap (Priority Queue), Merge Sort

---

## Problem

<p>You are given an array of <code>k</code> linked-lists <code>lists</code>, each linked-list is sorted in ascending order.</p>

<p><em>Merge all the linked-lists into one sorted linked-list and return it.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> lists = [[1,4,5],[1,3,4],[2,6]]
<strong>Output:</strong> [1,1,2,3,4,4,5,6]
<strong>Explanation:</strong> The linked-lists are:
[
  1-&gt;4-&gt;5,
  1-&gt;3-&gt;4,
  2-&gt;6
]
merging them into one sorted linked list:
1-&gt;1-&gt;2-&gt;3-&gt;4-&gt;4-&gt;5-&gt;6
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> lists = []
<strong>Output:</strong> []
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> lists = [[]]
<strong>Output:</strong> []
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>k == lists.length</code></li>
	<li><code>0 &lt;= k &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= lists[i].length &lt;= 500</code></li>
	<li><code>-10<sup>4</sup> &lt;= lists[i][j] &lt;= 10<sup>4</sup></code></li>
	<li><code>lists[i]</code> is sorted in <strong>ascending order</strong>.</li>
	<li>The sum of <code>lists[i].length</code> will not exceed <code>10<sup>4</sup></code>.</li>
</ul>



## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct BlackNode(Box<ListNode>);

impl Ord for BlackNode {
    fn cmp(&self, black_other: &Self) -> Ordering {
        black_other.0.val.cmp(&self.0.val)
    }
}

impl PartialOrd for BlackNode {
    fn partial_cmp(&self, black_other: &Self) -> Option<Ordering> {
        Some(self.cmp(black_other))
    }
}

impl Solution {
    pub fn merge_k_lists(black_lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut black_heap = BinaryHeap::new();
        for black_l in black_lists {
            if let Some(black_node) = black_l {
                black_heap.push(BlackNode(black_node));
            }
        }

        let mut black_dummy = Box::new(ListNode::new(0));
        let mut black_curr = &mut black_dummy;

        while let Some(BlackNode(mut black_node)) = black_heap.pop() {
            if let Some(black_next) = black_node.next.take() {
                black_heap.push(BlackNode(black_next));
            }
            black_curr.next = Some(black_node);
            black_curr = black_curr.next.as_mut().unwrap();
        }

        let bravexuneth = black_dummy.next;
        bravexuneth
    }
}
```