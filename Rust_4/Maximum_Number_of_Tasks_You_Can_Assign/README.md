# Maximum Number of Tasks You Can Assign

**Difficulty:** Hard
**Tags:** Array, Two Pointers, Binary Search, Greedy, Queue, Sorting, Monotonic Queue

---

## Problem

<p>You have <code>n</code> tasks and <code>m</code> workers. Each task has a strength requirement stored in a <strong>0-indexed</strong> integer array <code>tasks</code>, with the <code>i<sup>th</sup></code> task requiring <code>tasks[i]</code> strength to complete. The strength of each worker is stored in a <strong>0-indexed</strong> integer array <code>workers</code>, with the <code>j<sup>th</sup></code> worker having <code>workers[j]</code> strength. Each worker can only be assigned to a <strong>single</strong> task and must have a strength <strong>greater than or equal</strong> to the task&#39;s strength requirement (i.e., <code>workers[j] &gt;= tasks[i]</code>).</p>

<p>Additionally, you have <code>pills</code> magical pills that will <strong>increase a worker&#39;s strength</strong> by <code>strength</code>. You can decide which workers receive the magical pills, however, you may only give each worker <strong>at most one</strong> magical pill.</p>

<p>Given the <strong>0-indexed </strong>integer arrays <code>tasks</code> and <code>workers</code> and the integers <code>pills</code> and <code>strength</code>, return <em>the <strong>maximum</strong> number of tasks that can be completed.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> tasks = [<u><strong>3</strong></u>,<u><strong>2</strong></u>,<u><strong>1</strong></u>], workers = [<u><strong>0</strong></u>,<u><strong>3</strong></u>,<u><strong>3</strong></u>], pills = 1, strength = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong>
We can assign the magical pill and tasks as follows:
- Give the magical pill to worker 0.
- Assign worker 0 to task 2 (0 + 1 &gt;= 1)
- Assign worker 1 to task 1 (3 &gt;= 2)
- Assign worker 2 to task 0 (3 &gt;= 3)
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> tasks = [<u><strong>5</strong></u>,4], workers = [<u><strong>0</strong></u>,0,0], pills = 1, strength = 5
<strong>Output:</strong> 1
<strong>Explanation:</strong>
We can assign the magical pill and tasks as follows:
- Give the magical pill to worker 0.
- Assign worker 0 to task 0 (0 + 5 &gt;= 5)
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> tasks = [<u><strong>10</strong></u>,<u><strong>15</strong></u>,30], workers = [<u><strong>0</strong></u>,<u><strong>10</strong></u>,10,10,10], pills = 3, strength = 10
<strong>Output:</strong> 2
<strong>Explanation:</strong>
We can assign the magical pills and tasks as follows:
- Give the magical pill to worker 0 and worker 1.
- Assign worker 0 to task 0 (0 + 10 &gt;= 10)
- Assign worker 1 to task 1 (10 + 10 &gt;= 15)
The last pill is not given because it will not make any worker strong enough for the last task.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == tasks.length</code></li>
	<li><code>m == workers.length</code></li>
	<li><code>1 &lt;= n, m &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= pills &lt;= m</code></li>
	<li><code>0 &lt;= tasks[i], workers[j], strength &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Is it possible to assign the first k smallest tasks to the workers?
2. How can you efficiently try every k?

## Solution

```rust
use std::collections::BTreeMap;

impl Solution {
    pub fn max_task_assign(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        tasks.sort(); workers.sort();
        let (n, m) = (tasks.len(), workers.len());

        let check = |mid: usize| -> bool {
            let mut p = pills as usize;
            let mut avail: BTreeMap<i32, usize> = BTreeMap::new();
            for &w in &workers[m-mid..] { *avail.entry(w).or_insert(0) += 1; }
            for &t in tasks[..mid].iter().rev() {
                if let Some((&w, _)) = avail.range(t..).next() {
                    let c = avail.get_mut(&w).unwrap();
                    if *c == 1 { avail.remove(&w); } else { *c -= 1; }
                } else if p > 0 {
                    if let Some((&w, _)) = avail.range(t-strength..).next() {
                        let c = avail.get_mut(&w).unwrap();
                        if *c == 1 { avail.remove(&w); } else { *c -= 1; }
                        p -= 1;
                    } else { return false; }
                } else { return false; }
            }
            true
        };

        let (mut lo, mut hi) = (0, n.min(m));
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if check(mid) { lo = mid; } else { hi = mid - 1; }
        }
        lo as i32
    }
}
```