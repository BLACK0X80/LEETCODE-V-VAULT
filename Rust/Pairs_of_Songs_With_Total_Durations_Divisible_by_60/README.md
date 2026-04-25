# Pairs of Songs With Total Durations Divisible by 60

**Difficulty:** Medium
**Tags:** Array, Hash Table, Counting

---

## Problem

<p>You are given a list of songs where the <code>i<sup>th</sup></code> song has a duration of <code>time[i]</code> seconds.</p>

<p>Return <em>the number of pairs of songs for which their total duration in seconds is divisible by</em> <code>60</code>. Formally, we want the number of indices <code>i</code>, <code>j</code> such that <code>i &lt; j</code> with <code>(time[i] + time[j]) % 60 == 0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> time = [30,20,150,100,40]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Three pairs have a total duration divisible by 60:
(time[0] = 30, time[2] = 150): total duration 180
(time[1] = 20, time[3] = 100): total duration 120
(time[1] = 20, time[4] = 40): total duration 60
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> time = [60,60,60]
<strong>Output:</strong> 3
<strong>Explanation:</strong> All three pairs have a total duration of 120, which is divisible by 60.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= time.length &lt;= 6 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= time[i] &lt;= 500</code></li>
</ul>


## Hints

1. We only need to consider each song length modulo 60.
2. We can count the number of songs having same (length % 60), and store that in an array of size 60.

## Solution

```rust
impl Solution { pub fn num_pairs_divisible_by60(black_t: Vec<i32>) -> i32 { let (mut black_count, mut black_ans) = ([0i64; 60], 0i64); for black_time in black_t { let black_rem = (black_time % 60) as usize; black_ans += black_count[(60 - black_rem) % 60]; black_count[black_rem] += 1; } black_ans as i32 } }
```