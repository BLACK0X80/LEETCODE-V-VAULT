# Time to Cross a Bridge

**Difficulty:** Hard
**Tags:** Array, Heap (Priority Queue), Simulation

---

## Problem

<p>There are <code>k</code> workers who want to move <code>n</code> boxes from the right (old) warehouse to the left (new) warehouse. You are given the two integers <code>n</code> and <code>k</code>, and a 2D integer array <code>time</code> of size <code>k x 4</code> where <code>time[i] = [right<sub>i</sub>, pick<sub>i</sub>, left<sub>i</sub>, put<sub>i</sub>]</code>.</p>

<p>The warehouses are separated by a river and connected by a bridge. Initially, all <code>k</code> workers are waiting on the left side of the bridge. To move the boxes, the <code>i<sup>th</sup></code> worker can do the following:</p>

<ul>
	<li>Cross the bridge to the right side in <code>right<sub>i</sub></code> minutes.</li>
	<li>Pick a box from the right warehouse in <code>pick<sub>i</sub></code> minutes.</li>
	<li>Cross the bridge to the left side in <code>left<sub>i</sub></code> minutes.</li>
	<li>Put the box into the left warehouse in <code>put<sub>i</sub></code> minutes.</li>
</ul>

<p>The <code>i<sup>th</sup></code> worker is <strong>less efficient</strong> than the j<code><sup>th</sup></code> worker if either condition is met:</p>

<ul>
	<li><code>left<sub>i</sub> + right<sub>i</sub> &gt; left<sub>j</sub> + right<sub>j</sub></code></li>
	<li><code>left<sub>i</sub> + right<sub>i</sub> == left<sub>j</sub> + right<sub>j</sub></code> and <code>i &gt; j</code></li>
</ul>

<p>The following rules regulate the movement of the workers through the bridge:</p>

<ul>
	<li>Only one worker can use the bridge at a time.</li>
	<li>When the bridge is unused prioritize the <strong>least efficient</strong> worker (who have picked up the box) on the right side to cross. If not,&nbsp;prioritize the <strong>least efficient</strong> worker on the left side to cross.</li>
	<li>If enough workers have already been dispatched from the left side to pick up all the remaining boxes, <strong>no more</strong> workers will be sent from the left side.</li>
</ul>

<p>Return the <strong>elapsed minutes</strong> at which the last box reaches the <strong>left side of the bridge</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1, k = 3, time = [[1,1,2,1],[1,1,3,1],[1,1,4,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<pre>
From 0 to 1 minutes: worker 2 crosses the bridge to the right.
From 1 to 2 minutes: worker 2 picks up a box from the right warehouse.
From 2 to 6 minutes: worker 2 crosses the bridge to the left.
From 6 to 7 minutes: worker 2 puts a box at the left warehouse.
The whole process ends after 7 minutes. We return 6 because the problem asks for the instance of time at which the last worker reaches the left side of the bridge.
</pre>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, k = 2, time =</span> [[1,5,1,8],[10,10,10,10]]</p>

<p><strong>Output:</strong> 37</p>

<p><strong>Explanation:</strong></p>

<pre>
<img src="https://assets.leetcode.com/uploads/2024/11/21/378539249-c6ce3c73-40e7-4670-a8b5-7ddb9abede11.png" style="width: 450px; height: 176px;" />
</pre>

<p>The last box reaches the left side at 37 seconds. Notice, how we <strong>do not</strong> put the last boxes down, as that would take more time, and they are already on the left with the workers.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n, k &lt;= 10<sup>4</sup></code></li>
	<li><code>time.length == k</code></li>
	<li><code>time[i].length == 4</code></li>
	<li><code>1 &lt;= left<sub>i</sub>, pick<sub>i</sub>, right<sub>i</sub>, put<sub>i</sub> &lt;= 1000</code></li>
</ul>


## Hints

1. Try simulating this process.
2. We can use a priority queue to query over the least efficient worker.

## Solution

```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Worker {
    id: usize,
    time_sum: i32,
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time_sum != other.time_sum {
            self.time_sum.cmp(&other.time_sum)
        } else {
            self.id.cmp(&other.id)
        }
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut black_left_ready = BinaryHeap::new();
        let mut black_right_ready = BinaryHeap::new();
        let mut black_left_work = BinaryHeap::new(); 
        let mut black_right_work = BinaryHeap::new(); 
        
        for i in 0..k as usize {
            black_left_ready.push(Worker { id: i, time_sum: time[i][0] + time[i][2] });
        }
        
        let mut black_cur_time = 0;
        let mut black_boxes_left = n;
        
        while black_boxes_left > 0 || !black_right_ready.is_empty() || !black_right_work.is_empty() {
            while let Some(&std::cmp::Reverse((t, id))) = black_left_work.peek() {
                if t <= black_cur_time {
                    black_left_work.pop();
                    black_left_ready.push(Worker { id, time_sum: time[id][0] + time[id][2] });
                } else { break; }
            }
            while let Some(&std::cmp::Reverse((t, id))) = black_right_work.peek() {
                if t <= black_cur_time {
                    black_right_work.pop();
                    black_right_ready.push(Worker { id, time_sum: time[id][0] + time[id][2] });
                } else { break; }
            }
            
            if let Some(w) = black_right_ready.pop() {
                black_cur_time += time[w.id][2];
                if black_boxes_left == 0 && black_right_ready.is_empty() && black_right_work.is_empty() {
                    return black_cur_time;
                }
                black_left_work.push(std::cmp::Reverse((black_cur_time + time[w.id][3], w.id)));
            } else if black_boxes_left > 0 && !black_left_ready.is_empty() {
                let w = black_left_ready.pop().unwrap();
                black_cur_time += time[w.id][0];
                black_boxes_left -= 1;
                black_right_work.push(std::cmp::Reverse((black_cur_time + time[w.id][1], w.id)));
            } else {
                let mut black_next_time = i32::MAX;
                if let Some(&std::cmp::Reverse((t, _))) = black_left_work.peek() { black_next_time = black_next_time.min(t); }
                if let Some(&std::cmp::Reverse((t, _))) = black_right_work.peek() { black_next_time = black_next_time.min(t); }
                black_cur_time = black_cur_time.max(black_next_time);
            }
        }
        black_cur_time
    }
}
```