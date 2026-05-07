# Rotate List

**Difficulty:** Medium
**Tags:** Linked List, Two Pointers

---

## Problem

<p>Given the <code>head</code> of a linked&nbsp;list, rotate the list to the right by <code>k</code> places.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg" style="width: 450px; height: 191px;" />
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], k = 2
<strong>Output:</strong> [4,5,1,2,3]
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg" style="width: 305px; height: 350px;" />
<pre>
<strong>Input:</strong> head = [0,1,2], k = 4
<strong>Output:</strong> [2,0,1]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is in the range <code>[0, 500]</code>.</li>
	<li><code>-100 &lt;= Node.val &lt;= 100</code></li>
	<li><code>0 &lt;= k &lt;= 2 * 10<sup>9</sup></code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn rotate_right(mut black_h: Option<Box<ListNode>>, black_k: i32) -> Option<Box<ListNode>> {
        if black_h.is_none() || black_k == 0 { return black_h; }
        let mut black_n = 0;
        let mut black_curr = &black_h;
        while let Some(node) = black_curr { black_n += 1; black_curr = &node.next; }
        let black_k = black_k % black_n;
        if black_k == 0 { return black_h; }
        let mut black_curr = black_h.as_mut().unwrap();
        for _ in 0..black_n - black_k - 1 { black_curr = black_curr.next.as_mut().unwrap(); }
        let mut black_new_h = black_curr.next.take();
        let mut black_tail = black_new_h.as_mut().unwrap();
        while black_tail.next.is_some() { black_tail = black_tail.next.as_mut().unwrap(); }
        black_tail.next = black_h;
        black_new_h
    }
}
```