# Reverse Linked List II

**Difficulty:** Medium
**Tags:** Linked List

---

## Problem

<p>Given the <code>head</code> of a singly linked list and two integers <code>left</code> and <code>right</code> where <code>left &lt;= right</code>, reverse the nodes of the list from position <code>left</code> to position <code>right</code>, and return <em>the reversed list</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev2ex2.jpg" style="width: 542px; height: 222px;" />
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], left = 2, right = 4
<strong>Output:</strong> [1,4,3,2,5]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> head = [5], left = 1, right = 1
<strong>Output:</strong> [5]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li>The number of nodes in the list is <code>n</code>.</li>
	<li><code>1 &lt;= n &lt;= 500</code></li>
	<li><code>-500 &lt;= Node.val &lt;= 500</code></li>
	<li><code>1 &lt;= left &lt;= right &lt;= n</code></li>
</ul>

<p>&nbsp;</p>
<strong>Follow up:</strong> Could you do it in one pass?


## Solution

```rust
impl Solution {
    pub fn reverse_between(mut black_h: Option<Box<ListNode>>, black_l: i32, black_r: i32) -> Option<Box<ListNode>> {
        let mut black_dummy = Some(Box::new(ListNode { val: 0, next: black_h }));
        let mut black_pre = black_dummy.as_mut().unwrap();
        for _ in 1..black_l { black_pre = black_pre.next.as_mut().unwrap(); }
        let mut black_curr = black_pre.next.take();
        for _ in 0..black_r - black_l {
            let mut black_next = black_curr.as_mut().unwrap().next.take();
            black_curr.as_mut().unwrap().next = black_next.as_mut().unwrap().next.take();
            black_next.as_mut().unwrap().next = black_pre.next.take();
            black_pre.next = black_next;
        }
        let mut black_tail = black_pre;
        while black_tail.next.is_some() { black_tail = black_tail.next.as_mut().unwrap(); }
        black_tail.next = black_curr;
        black_dummy.unwrap().next
    }
}
```