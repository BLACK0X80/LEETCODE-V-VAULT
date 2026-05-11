# Maximum Number of Robots Within Budget

**Difficulty:** Hard
**Tags:** Array, Binary Search, Queue, Sliding Window, Heap (Priority Queue), Prefix Sum, Monotonic Queue

---

## Problem

<p>You have <code>n</code> robots. You are given two <strong>0-indexed</strong> integer arrays, <code>chargeTimes</code> and <code>runningCosts</code>, both of length <code>n</code>. The <code>i<sup>th</sup></code> robot costs <code>chargeTimes[i]</code> units to charge and costs <code>runningCosts[i]</code> units to run. You are also given an integer <code>budget</code>.</p>

<p>The <strong>total cost</strong> of running <code>k</code> chosen robots is equal to <code>max(chargeTimes) + k * sum(runningCosts)</code>, where <code>max(chargeTimes)</code> is the largest charge cost among the <code>k</code> robots and <code>sum(runningCosts)</code> is the sum of running costs among the <code>k</code> robots.</p>

<p>Return<em> the <strong>maximum</strong> number of <strong>consecutive</strong> robots you can run such that the total cost <strong>does not</strong> exceed </em><code>budget</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> chargeTimes = [3,6,1,3,4], runningCosts = [2,1,3,4,5], budget = 25
<strong>Output:</strong> 3
<strong>Explanation:</strong> 
It is possible to run all individual and consecutive pairs of robots within budget.
To obtain answer 3, consider the first 3 robots. The total cost will be max(3,6,1) + 3 * sum(2,1,3) = 6 + 3 * 6 = 24 which is less than 25.
It can be shown that it is not possible to run more than 3 consecutive robots within budget, so we return 3.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> chargeTimes = [11,12,19], runningCosts = [10,8,7], budget = 19
<strong>Output:</strong> 0
<strong>Explanation:</strong> No robot can be run that does not exceed the budget, so we return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>chargeTimes.length == runningCosts.length == n</code></li>
	<li><code>1 &lt;= n &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= chargeTimes[i], runningCosts[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= budget &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. Use binary search to convert the problem into checking if we can find a specific number of consecutive robots within the budget.
2. Maintain a sliding window of the consecutive robots being considered.
3. Use either a map, deque, or heap to find the maximum charge times in the window efficiently.

## Solution

```rust
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut black_dq = VecDeque::new();
        let mut black_sum_run = 0i64;
        let mut black_left = 0;
        let mut black_max_robots = 0;

        for black_right in 0..charge_times.len() {
            while !black_dq.is_empty() && charge_times[*black_dq.back().unwrap()] <= charge_times[black_right] {
                black_dq.pop_back();
            }
            black_dq.push_back(black_right);
            black_sum_run += running_costs[black_right] as i64;

            while !black_dq.is_empty() && (charge_times[*black_dq.front().unwrap()] as i64 + (black_right - black_left + 1) as i64 * black_sum_run) > budget {
                if *black_dq.front().unwrap() == black_left {
                    black_dq.pop_front();
                }
                black_sum_run -= running_costs[black_left] as i64;
                black_left += 1;
            }
            black_max_robots = black_max_robots.max(black_right - black_left + 1);
        }
        black_max_robots as i32
    }
}
```