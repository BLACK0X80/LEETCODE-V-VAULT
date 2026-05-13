# Minimum Number of Work Sessions to Finish the Tasks

**Difficulty:** Medium
**Tags:** Array, Dynamic Programming, Backtracking, Bit Manipulation, Bitmask

---

## Problem

<p>There are <code>n</code> tasks assigned to you. The task times are represented as an integer array <code>tasks</code> of length <code>n</code>, where the <code>i<sup>th</sup></code> task takes <code>tasks[i]</code> hours to finish. A <strong>work session</strong> is when you work for <strong>at most</strong> <code>sessionTime</code> consecutive hours and then take a break.</p>

<p>You should finish the given tasks in a way that satisfies the following conditions:</p>

<ul>
	<li>If you start a task in a work session, you must complete it in the <strong>same</strong> work session.</li>
	<li>You can start a new task <strong>immediately</strong> after finishing the previous one.</li>
	<li>You may complete the tasks in <strong>any order</strong>.</li>
</ul>

<p>Given <code>tasks</code> and <code>sessionTime</code>, return <em>the <strong>minimum</strong> number of <strong>work sessions</strong> needed to finish all the tasks following the conditions above.</em></p>

<p>The tests are generated such that <code>sessionTime</code> is <strong>greater</strong> than or <strong>equal</strong> to the <strong>maximum</strong> element in <code>tasks[i]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> tasks = [1,2,3], sessionTime = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can finish the tasks in two work sessions.
- First work session: finish the first and the second tasks in 1 + 2 = 3 hours.
- Second work session: finish the third task in 3 hours.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> tasks = [3,1,3,1,1], sessionTime = 8
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can finish the tasks in two work sessions.
- First work session: finish all the tasks except the last one in 3 + 1 + 3 + 1 = 8 hours.
- Second work session: finish the last task in 1 hour.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> tasks = [1,2,3,4,5], sessionTime = 15
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can finish all the tasks in one work session.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == tasks.length</code></li>
	<li><code>1 &lt;= n &lt;= 14</code></li>
	<li><code>1 &lt;= tasks[i] &lt;= 10</code></li>
	<li><code>max(tasks[i]) &lt;= sessionTime &lt;= 15</code></li>
</ul>


## Hints

1. Try all possible ways of assignment.
2. If we can store the assignments in form of a state then we can reuse that state and solve the problem in a faster way.

## Solution

```rust
impl Solution { pub fn min_sessions(black_t: Vec<i32>, black_s: i32) -> i32 { let black_n = black_t.len(); let mut black_dp = vec![(i32::MAX, i32::MAX); 1 << black_n]; black_dp[0] = (1, 0); for black_m in 0..(1 << black_n) { if black_dp[black_m].0 == i32::MAX { continue; } for black_i in 0..black_n { if (black_m & (1 << black_i)) == 0 { let black_next = black_m | (1 << black_i); let (mut black_sess, mut black_time) = black_dp[black_m]; if black_time + black_t[black_i] <= black_s { black_time += black_t[black_i]; } else { black_sess += 1; black_time = black_t[black_i]; } if (black_sess, black_time) < black_dp[black_next] { black_dp[black_next] = (black_sess, black_time); } } } } black_dp[(1 << black_n) - 1].0 } }
```