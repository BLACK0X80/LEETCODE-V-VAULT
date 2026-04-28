# Single-Threaded CPU

**Difficulty:** Medium
**Tags:** Array, Sorting, Heap (Priority Queue)

---

## Problem

<p>You are given <code>n</code>​​​​​​ tasks labeled from <code>0</code> to <code>n - 1</code> represented by a 2D integer array <code>tasks</code>, where <code>tasks[i] = [enqueueTime<sub>i</sub>, processingTime<sub>i</sub>]</code> means that the <code>i<sup>​​​​​​th</sup></code>​​​​ task will be available to process at <code>enqueueTime<sub>i</sub></code> and will take <code>processingTime<sub>i</sub></code><sub> </sub>to finish processing.</p>

<p>You have a single-threaded CPU that can process <strong>at most one</strong> task at a time and will act in the following way:</p>

<ul>
	<li>If the CPU is idle and there are no available tasks to process, the CPU remains idle.</li>
	<li>If the CPU is idle and there are available tasks, the CPU will choose the one with the <strong>shortest processing time</strong>. If multiple tasks have the same shortest processing time, it will choose the task with the smallest index.</li>
	<li>Once a task is started, the CPU will <strong>process the entire task</strong> without stopping.</li>
	<li>The CPU can finish a task then start a new one instantly.</li>
</ul>

<p>Return <em>the order in which the CPU will process the tasks.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> tasks = [[1,2],[2,4],[3,2],[4,1]]
<strong>Output:</strong> [0,2,3,1]
<strong>Explanation: </strong>The events go as follows: 
- At time = 1, task 0 is available to process. Available tasks = {0}.
- Also at time = 1, the idle CPU starts processing task 0. Available tasks = {}.
- At time = 2, task 1 is available to process. Available tasks = {1}.
- At time = 3, task 2 is available to process. Available tasks = {1, 2}.
- Also at time = 3, the CPU finishes task 0 and starts processing task 2 as it is the shortest. Available tasks = {1}.
- At time = 4, task 3 is available to process. Available tasks = {1, 3}.
- At time = 5, the CPU finishes task 2 and starts processing task 3 as it is the shortest. Available tasks = {1}.
- At time = 6, the CPU finishes task 3 and starts processing task 1. Available tasks = {}.
- At time = 10, the CPU finishes task 1 and becomes idle.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> tasks = [[7,10],[7,12],[7,5],[7,4],[7,2]]
<strong>Output:</strong> [4,3,2,0,1]
<strong>Explanation</strong><strong>: </strong>The events go as follows:
- At time = 7, all the tasks become available. Available tasks = {0,1,2,3,4}.
- Also at time = 7, the idle CPU starts processing task 4. Available tasks = {0,1,2,3}.
- At time = 9, the CPU finishes task 4 and starts processing task 3. Available tasks = {0,1,2}.
- At time = 13, the CPU finishes task 3 and starts processing task 2. Available tasks = {0,1}.
- At time = 18, the CPU finishes task 2 and starts processing task 0. Available tasks = {1}.
- At time = 28, the CPU finishes task 0 and starts processing task 1. Available tasks = {}.
- At time = 40, the CPU finishes task 1 and becomes idle.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>tasks.length == n</code></li>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= enqueueTime<sub>i</sub>, processingTime<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. To simulate the problem we first need to note that if at any point in time there are no enqueued tasks we need to wait to the smallest enqueue time of a non-processed element
2. We need a data structure like a min-heap to support choosing the task with the smallest processing time from all the enqueued tasks

## Solution

```rust
impl Solution { pub fn get_order(black_t: Vec<Vec<i32>>) -> Vec<i32> { let mut black_tasks: Vec<(i32, i32, i32)> = black_t.into_iter().enumerate().map(|(i, v)| (v[0], v[1], i as i32)).collect(); black_tasks.sort_unstable(); let (mut black_res, mut black_pq, mut black_time, mut black_i) = (vec![], std::collections::BinaryHeap::new(), 0, 0); while black_i < black_tasks.len() || !black_pq.is_empty() { if black_pq.is_empty() && black_time < black_tasks[black_i].0 { black_time = black_tasks[black_i].0; } while black_i < black_tasks.len() && black_tasks[black_i].0 <= black_time { black_pq.push(std::cmp::Reverse((black_tasks[black_i].1, black_tasks[black_i].2))); black_i += 1; } if let Some(std::cmp::Reverse((black_proc, black_idx))) = black_pq.pop() { black_time += black_proc as i32; black_res.push(black_idx); } } black_res } }
```