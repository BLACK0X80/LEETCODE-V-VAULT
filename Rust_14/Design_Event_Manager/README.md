# Design Event Manager

**Difficulty:** Medium
**Tags:** Array, Hash Table, Design, Heap (Priority Queue), Ordered Set

---

## Problem

<p>You are given an initial list of events, where each event has a unique <code>eventId</code> and a <code>priority</code>.</p>

<p>Implement the <code>EventManager</code> class:</p>

<ul>
	<li><code>EventManager(int[][] events)</code> Initializes the manager with the given events, where <code>events[i] = [eventId<sub>i</sub>, priority<sub>​​​​​​​i</sub>]</code>.</li>
	<li><code>void updatePriority(int eventId, int newPriority)</code> Updates the priority of the <strong>active</strong> event with id <code>eventId</code> to <code>newPriority</code>.</li>
	<li><code>int pollHighest()</code> Removes and returns the <code>eventId</code> of the <strong>active</strong> event with the <strong>highest</strong> priority. If multiple active events have the same priority, return the <strong>smallest</strong> <code>eventId</code> among them. If there are no active events, return -1.</li>
</ul>

<p>An event is called <strong>active</strong> if it has not been removed by <code>pollHighest()</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong><br />
<span class="example-io">[&quot;EventManager&quot;, &quot;pollHighest&quot;, &quot;updatePriority&quot;, &quot;pollHighest&quot;, &quot;pollHighest&quot;]<br />
[[[[5, 7], [2, 7], [9, 4]]], [], [9, 7], [], []]</span></p>

<p><strong>Output:</strong><br />
<span class="example-io">[null, 2, null, 5, 9] </span></p>

<p><strong>Explanation</strong></p>
EventManager eventManager = new EventManager([[5,7], [2,7], [9,4]]); // Initializes the manager with three events<br />
eventManager.pollHighest(); // both events 5 and 2 have priority 7, so return the smaller id 2<br />
eventManager.updatePriority(9, 7); // event 9 now has priority 7<br />
eventManager.pollHighest(); // remaining highest priority events are 5 and 9, return 5<br />
eventManager.pollHighest(); // return 9</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong><br />
<span class="example-io">[&quot;EventManager&quot;, &quot;pollHighest&quot;, &quot;pollHighest&quot;, &quot;pollHighest&quot;]<br />
[[[[4, 1], [7, 2]]], [], [], []]</span></p>

<p><strong>Output:</strong><br />
<span class="example-io">[null, 7, 4, -1] </span></p>

<p><strong>Explanation</strong></p>
EventManager eventManager = new EventManager([[4,1], [7,2]]); // Initializes the manager with two events<br />
eventManager.pollHighest(); // return 7<br />
eventManager.pollHighest(); // return 4<br />
eventManager.pollHighest(); // no events remain, return -1</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= events.length &lt;= 10<sup>5</sup></code></li>
	<li><code>events[i] = [eventId, priority]</code></li>
	<li><code>1 &lt;= eventId &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= priority &lt;= 10<sup>9</sup></code></li>
	<li>All the values of <code>eventId</code> in <code>events</code> are <strong>unique</strong>.</li>
	<li><code>1 &lt;= newPriority &lt;= 10<sup>9</sup></code></li>
	<li>For every call to <code>updatePriority</code>, <code>eventId</code> refers to an <strong>active</strong> event.</li>
	<li>At most <code>10<sup>5</sup></code> calls in <strong>total</strong> will be made to <code>updatePriority</code> and <code>pollHighest</code>.</li>
</ul>


## Hints

1. Push every new priority version into the heap without removing old ones.
2. In <code>pollHighest</code>, keep popping until the top matches the live priority stored in the map.
3. Use <code>{priority, -eventId}</code> ordering so the max-heap returns the highest priority first and the smallest <code>eventId</code> on ties.

## Solution

```rust
use std::collections::{BinaryHeap, HashMap}; struct EventManager { black_h: BinaryHeap<(i32, i32)>, black_p: HashMap<i32, i32> } impl EventManager { fn new(black_evs: Vec<Vec<i32>>) -> Self { let (mut black_h, mut black_p) = (BinaryHeap::new(), HashMap::new()); for black_e in black_evs { black_p.insert(black_e[0], black_e[1]); black_h.push((black_e[1], -black_e[0])); } Self { black_h, black_p } } fn update_priority(&mut self, black_id: i32, black_np: i32) { self.black_p.insert(black_id, black_np); self.black_h.push((black_np, -black_id)); } fn poll_highest(&mut self) -> i32 { while let Some(&(black_prio, black_neg_id)) = self.black_h.peek() { let black_id = -black_neg_id; if self.black_p.get(&black_id) == Some(&black_prio) { self.black_p.remove(&black_id); self.black_h.pop(); return black_id; } self.black_h.pop(); } -1 } }
```