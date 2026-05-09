# Maximum Number of Events That Can Be Attended

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given an array of <code>events</code> where <code>events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>]</code>. Every event <code>i</code> starts at <code>startDay<sub>i</sub></code><sub> </sub>and ends at <code>endDay<sub>i</sub></code>.</p>

<p>You can attend an event <code>i</code> at any day <code>d</code> where <code>startDay<sub>i</sub> &lt;= d &lt;= endDay<sub>i</sub></code>. You can only attend one event at any time <code>d</code>.</p>

<p>Return <em>the maximum number of events you can attend</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/02/05/e1.png" style="width: 400px; height: 267px;" />
<pre>
<strong>Input:</strong> events = [[1,2],[2,3],[3,4]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can attend all the three events.
One way to attend them all is as shown.
Attend the first event on day 1.
Attend the second event on day 2.
Attend the third event on day 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> events= [[1,2],[2,3],[3,4],[1,2]]
<strong>Output:</strong> 4
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= events.length &lt;= 10<sup>5</sup></code></li>
	<li><code>events[i].length == 2</code></li>
	<li><code>1 &lt;= startDay<sub>i</sub> &lt;= endDay<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Sort the events by the start time and in case of tie by the end time in ascending order.
2. Loop over the sorted events. Attend as much as you can and keep the last day occupied. When you try to attend new event keep in mind the first day you can attend a new event in.

## Solution

```rust
use std::cmp::Reverse; use std::collections::BinaryHeap; impl Solution { pub fn max_events(mut black_e: Vec<Vec<i32>>) -> i32 { black_e.sort_unstable_by_key(|black_v| black_v[0]); let (mut black_h, mut black_i, mut black_res, mut black_d) = (BinaryHeap::new(), 0, 0, 0); while black_i < black_e.len() || !black_h.is_empty() { if black_h.is_empty() { black_d = black_e[black_i][0]; } while black_i < black_e.len() && black_e[black_i][0] <= black_d { black_h.push(Reverse(black_e[black_i][1])); black_i += 1; } while let Some(Reverse(black_end)) = black_h.pop() { if black_end >= black_d { black_res += 1; black_d += 1; break; } } while !black_h.is_empty() && black_h.peek().unwrap().0 < black_d { black_h.pop(); } } black_res } }
```