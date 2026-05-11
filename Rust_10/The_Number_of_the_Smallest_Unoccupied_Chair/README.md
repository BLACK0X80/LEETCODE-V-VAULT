# The Number of the Smallest Unoccupied Chair

**Difficulty:** Medium
**Tags:** Array, Hash Table, Heap (Priority Queue)

---

## Problem

<p>There is a party where <code>n</code> friends numbered from <code>0</code> to <code>n - 1</code> are attending. There is an <strong>infinite</strong> number of chairs in this party that are numbered from <code>0</code> to <code>infinity</code>. When a friend arrives at the party, they sit on the unoccupied chair with the <strong>smallest number</strong>.</p>

<ul>
	<li>For example, if chairs <code>0</code>, <code>1</code>, and <code>5</code> are occupied when a friend comes, they will sit on chair number <code>2</code>.</li>
</ul>

<p>When a friend leaves the party, their chair becomes unoccupied at the moment they leave. If another friend arrives at that same moment, they can sit in that chair.</p>

<p>You are given a <strong>0-indexed</strong> 2D integer array <code>times</code> where <code>times[i] = [arrival<sub>i</sub>, leaving<sub>i</sub>]</code>, indicating the arrival and leaving times of the <code>i<sup>th</sup></code> friend respectively, and an integer <code>targetFriend</code>. All arrival times are <strong>distinct</strong>.</p>

<p>Return<em> the <strong>chair number</strong> that the friend numbered </em><code>targetFriend</code><em> will sit on</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> times = [[1,4],[2,3],[4,6]], targetFriend = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> 
- Friend 0 arrives at time 1 and sits on chair 0.
- Friend 1 arrives at time 2 and sits on chair 1.
- Friend 1 leaves at time 3 and chair 1 becomes empty.
- Friend 0 leaves at time 4 and chair 0 becomes empty.
- Friend 2 arrives at time 4 and sits on chair 0.
Since friend 1 sat on chair 1, we return 1.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> times = [[3,10],[1,5],[2,6]], targetFriend = 0
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
- Friend 1 arrives at time 1 and sits on chair 0.
- Friend 2 arrives at time 2 and sits on chair 1.
- Friend 0 arrives at time 3 and sits on chair 2.
- Friend 1 leaves at time 5 and chair 0 becomes empty.
- Friend 2 leaves at time 6 and chair 1 becomes empty.
- Friend 0 leaves at time 10 and chair 2 becomes empty.
Since friend 0 sat on chair 2, we return 2.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == times.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>times[i].length == 2</code></li>
	<li><code>1 &lt;= arrival<sub>i</sub> &lt; leaving<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= targetFriend &lt;= n - 1</code></li>
	<li>Each <code>arrival<sub>i</sub></code> time is <strong>distinct</strong>.</li>
</ul>


## Hints

1. Sort times by arrival time.
2. for each arrival_i find the smallest unoccupied chair and mark it as occupied until leaving_i.

## Solution

```rust
impl Solution { pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 { let mut black_events: Vec<(i32, i32, i32)> = times.iter().enumerate().flat_map(|(black_i, black_t)| vec![(black_t[0], 1, black_i as i32), (black_t[1], 0, black_i as i32)]).collect(); black_events.sort(); let (mut black_occupied, mut black_fix_heap) = (std::collections::BinaryHeap::<(std::cmp::Reverse<i32>, i32)>::new(), (0..times.len() as i32).map(|black_x| -black_x).collect::<std::collections::BinaryHeap<i32>>()); for (black_t, black_type, black_id) in black_events { while let Some(&(std::cmp::Reverse(black_time), _)) = black_occupied.peek() { if black_time <= black_t { if let Some((_, black_c)) = black_occupied.pop() { black_fix_heap.push(-black_c); } } else { break; } } if black_type == 1 { let black_chair = -black_fix_heap.pop().unwrap(); if black_id == target_friend { return black_chair; } black_occupied.push((std::cmp::Reverse(times[black_id as usize][1]), black_chair)); } } 0 } }
```