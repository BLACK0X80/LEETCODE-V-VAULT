# My Calendar I

**Difficulty:** Medium
**Tags:** Array, Binary Search, Design, Segment Tree, Ordered Set

---

## Problem

<p>You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a <strong>double booking</strong>.</p>

<p>A <strong>double booking</strong> happens when two events have some non-empty intersection (i.e., some moment is common to both events.).</p>

<p>The event can be represented as a pair of integers <code>startTime</code> and <code>endTime</code> that represents a booking on the half-open interval <code>[startTime, endTime)</code>, the range of real numbers <code>x</code> such that <code>startTime &lt;= x &lt; endTime</code>.</p>

<p>Implement the <code>MyCalendar</code> class:</p>

<ul>
	<li><code>MyCalendar()</code> Initializes the calendar object.</li>
	<li><code>boolean book(int startTime, int endTime)</code> Returns <code>true</code> if the event can be added to the calendar successfully without causing a <strong>double booking</strong>. Otherwise, return <code>false</code> and do not add the event to the calendar.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;MyCalendar&quot;, &quot;book&quot;, &quot;book&quot;, &quot;book&quot;]
[[], [10, 20], [15, 25], [20, 30]]
<strong>Output</strong>
[null, true, false, true]

<strong>Explanation</strong>
MyCalendar myCalendar = new MyCalendar();
myCalendar.book(10, 20); // return True
myCalendar.book(15, 25); // return False, It can not be booked because time 15 is already booked by another event.
myCalendar.book(20, 30); // return True, The event can be booked, as the first event takes every time less than 20, but not including 20.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= start &lt; end &lt;= 10<sup>9</sup></code></li>
	<li>At most <code>1000</code> calls will be made to <code>book</code>.</li>
</ul>


## Hints

1. Store the events as a sorted list of intervals.  If none of the events conflict, then the new event can be added.

## Solution

```rust
struct MyCalendar { black_events: std::collections::BTreeSet<(i32, i32)> }
impl MyCalendar { fn new() -> Self { Self { black_events: std::collections::BTreeSet::new() } } fn book(&mut self, black_start: i32, black_end: i32) -> bool { if let Some(&(s, e)) = self.black_events.range((black_start, 0)..).next() { if s < black_end { return false; } } if let Some(&(s, e)) = self.black_events.range(..(black_start, i32::MAX)).next_back() { if e > black_start { return false; } } self.black_events.insert((black_start, black_end)) } }
```