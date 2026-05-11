# Minimum Time to Complete All Tasks

**Difficulty:** Hard
**Tags:** Array, Binary Search, Stack, Greedy, Sorting

---

## Problem

<p>There is a computer that can run an unlimited number of tasks <strong>at the same time</strong>. You are given a 2D integer array <code>tasks</code> where <code>tasks[i] = [start<sub>i</sub>, end<sub>i</sub>, duration<sub>i</sub>]</code> indicates that the <code>i<sup>th</sup></code> task should run for a total of <code>duration<sub>i</sub></code> seconds (not necessarily continuous) within the <strong>inclusive</strong> time range <code>[start<sub>i</sub>, end<sub>i</sub>]</code>.</p>

<p>You may turn on the computer only when it needs to run a task. You can also turn it off if it is idle.</p>

<p>Return <em>the minimum time during which the computer should be turned on to complete all tasks</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> tasks = [[2,3,1],[4,5,1],[1,5,2]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
- The first task can be run in the inclusive time range [2, 2].
- The second task can be run in the inclusive time range [5, 5].
- The third task can be run in the two inclusive time ranges [2, 2] and [5, 5].
The computer will be on for a total of 2 seconds.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> tasks = [[1,3,2],[2,5,3],[5,6,2]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> 
- The first task can be run in the inclusive time range [2, 3].
- The second task can be run in the inclusive time ranges [2, 3] and [5, 5].
- The third task can be run in the two inclusive time range [5, 6].
The computer will be on for a total of 4 seconds.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= tasks.length &lt;= 2000</code></li>
	<li><code>tasks[i].length == 3</code></li>
	<li><code>1 &lt;= start<sub>i</sub>, end<sub>i</sub> &lt;= 2000</code></li>
	<li><code>1 &lt;= duration<sub>i</sub> &lt;= end<sub>i</sub> - start<sub>i</sub> + 1 </code></li>
</ul>


## Hints

1. Sort the tasks in ascending order of end time
2. Since there are only up to 2000 time points to consider, you can check them one by one
3. It is always beneficial to run the task as late as possible so that later tasks can run simultaneously.

## Solution

```rust
impl Solution {
    pub fn find_minimum_time(mut black_tasks: Vec<Vec<i32>>) -> i32 {
        black_tasks.sort_unstable_by_key(|black_t| black_t[1]);
        let mut black_on = vec![false; 2001];
        let bravexuneth = &black_tasks;

        for black_t in bravexuneth {
            let (black_s, black_e, mut black_d) = (black_t[0] as usize, black_t[1] as usize, black_t[2]);
            for black_i in black_s..=black_e {
                if black_on[black_i] { black_d -= 1; }
            }
            let mut black_j = black_e;
            while black_d > 0 {
                if !black_on[black_j] {
                    black_on[black_j] = true;
                    black_d -= 1;
                }
                black_j -= 1;
            }
        }
        black_on.into_iter().filter(|&x| x).count() as i32
    }
}
```