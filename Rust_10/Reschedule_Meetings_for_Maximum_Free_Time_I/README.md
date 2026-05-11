# Reschedule Meetings for Maximum Free Time I

**Difficulty:** Medium
**Tags:** Array, Greedy, Sliding Window

---

## Problem

<p>You are given an integer <code>eventTime</code> denoting the duration of an event, where the event occurs from time <code>t = 0</code> to time <code>t = eventTime</code>.</p>

<p>You are also given two integer arrays <code>startTime</code> and <code>endTime</code>, each of length <code>n</code>. These represent the start and end time of <code>n</code> <strong>non-overlapping</strong> meetings, where the <code>i<sup>th</sup></code> meeting occurs during the time <code>[startTime[i], endTime[i]]</code>.</p>

<p>You can reschedule <strong>at most</strong> <code>k</code> meetings by moving their start time while maintaining the <strong>same duration</strong>, to <strong>maximize</strong> the <strong>longest</strong> <em>continuous period of free time</em> during the event.</p>

<p>The <strong>relative</strong> order of all the meetings should stay the<em> same</em> and they should remain non-overlapping.</p>

<p>Return the <strong>maximum</strong> amount of free time possible after rearranging the meetings.</p>

<p><strong>Note</strong> that the meetings can <strong>not</strong> be rescheduled to a time outside the event.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">eventTime = 5, k = 1, startTime = [1,3], endTime = [2,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/21/example0_rescheduled.png" style="width: 375px; height: 123px;" /></p>

<p>Reschedule the meeting at <code>[1, 2]</code> to <code>[2, 3]</code>, leaving no meetings during the time <code>[0, 2]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">eventTime = 10, k = 1, startTime = [0,2,9], endTime = [1,4,10]</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2024/12/21/example1_rescheduled.png" style="width: 375px; height: 125px;" /></p>

<p>Reschedule the meeting at <code>[2, 4]</code> to <code>[1, 3]</code>, leaving no meetings during the time <code>[3, 9]</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">eventTime = 5, k = 2, startTime = [0,1,2,3,4], endTime = [1,2,3,4,5]</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>

<p><strong>Explanation:</strong></p>

<p>There is no time during the event not occupied by meetings.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= eventTime &lt;= 10<sup>9</sup></code></li>
	<li><code>n == startTime.length == endTime.length</code></li>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= n</code></li>
	<li><code>0 &lt;= startTime[i] &lt; endTime[i] &lt;= eventTime</code></li>
	<li><code>endTime[i] &lt;= startTime[i + 1]</code> where <code>i</code> lies in the range <code>[0, n - 2]</code>.</li>
</ul>


## Hints

1. In a sequence of <code>K</code> meetings and <code>K + 1</code> gaps, you could move all meetings to the start of the sequence to get the max free time.
2. Use a sliding window of <code>K + 1</code> size to store sum of gaps and take the maximum.

## Solution

```rust
impl Solution { pub fn max_free_time(black_et: i32, black_k: i32, black_st: Vec<i32>, black_et_arr: Vec<i32>) -> i32 { let mut black_gaps = vec![black_st[0]]; for i in 1..black_st.len() { black_gaps.push(black_st[i] - black_et_arr[i-1]); } black_gaps.push(black_et - black_et_arr[black_et_arr.len()-1]); let (mut black_ans, mut black_sum, black_win) = (0, 0, black_k as usize + 1); for i in 0..black_gaps.len() { black_sum += black_gaps[i]; if i >= black_win { black_sum -= black_gaps[i - black_win]; } black_ans = black_ans.max(black_sum); } black_ans } }
```