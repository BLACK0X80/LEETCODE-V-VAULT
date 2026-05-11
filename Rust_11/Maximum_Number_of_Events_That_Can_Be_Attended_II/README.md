# Maximum Number of Events That Can Be Attended II

**Difficulty:** Hard
**Tags:** Array, Binary Search, Dynamic Programming, Sorting

---

## Problem

<p>You are given an array of <code>events</code> where <code>events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>, value<sub>i</sub>]</code>. The <code>i<sup>th</sup></code> event starts at <code>startDay<sub>i</sub></code><sub> </sub>and ends at <code>endDay<sub>i</sub></code>, and if you attend this event, you will receive a value of <code>value<sub>i</sub></code>. You are also given an integer <code>k</code> which represents the maximum number of events you can attend.</p>

<p>You can only attend one event at a time. If you choose to attend an event, you must attend the <strong>entire</strong> event. Note that the end day is <strong>inclusive</strong>: that is, you cannot attend two events where one of them starts and the other ends on the same day.</p>

<p>Return <em>the <strong>maximum sum</strong> of values that you can receive by attending events.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60048-pm.png" style="width: 400px; height: 103px;" /></p>

<pre>
<strong>Input:</strong> events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
<strong>Output:</strong> 7
<strong>Explanation: </strong>Choose the green events, 0 and 1 (0-indexed) for a total value of 4 + 3 = 7.</pre>

<p><strong class="example">Example 2:</strong></p>

<p><img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60150-pm.png" style="width: 400px; height: 103px;" /></p>

<pre>
<strong>Input:</strong> events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
<strong>Output:</strong> 10
<strong>Explanation:</strong> Choose event 2 for a total value of 10.
Notice that you cannot attend any other event as they overlap, and that you do <strong>not</strong> have to attend k events.</pre>

<p><strong class="example">Example 3:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60703-pm.png" style="width: 400px; height: 126px;" /></strong></p>

<pre>
<strong>Input:</strong> events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
<strong>Output:</strong> 9
<strong>Explanation:</strong> Although the events do not overlap, you can only attend 3 events. Pick the highest valued three.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= events.length</code></li>
	<li><code>1 &lt;= k * events.length &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= startDay<sub>i</sub> &lt;= endDay<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= value<sub>i</sub> &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Sort the events by its startTime.
2. For every event, you can either choose it and consider the next event available, or you can ignore it. You can efficiently find the next event that is available using binary search.

## Solution

```rust
impl Solution {
    pub fn max_value(mut black_events: Vec<Vec<i32>>, black_k: i32) -> i32 {
        black_events.sort_by_key(|black_e| black_e[0]);
        let black_n = black_events.len();
        let mut black_dp = vec![vec![-1; black_k as usize + 1]; black_n];

        let bravexuneth = black_events;
        Self::black_solve(0, black_k as usize, &bravexuneth, &mut black_dp)
    }

    fn black_solve(black_i: usize, black_rem: usize, black_evs: &Vec<Vec<i32>>, black_dp: &mut Vec<Vec<i32>>) -> i32 {
        if black_i >= black_evs.len() || black_rem == 0 { return 0; }
        if black_dp[black_i][black_rem] != -1 { return black_dp[black_i][black_rem]; }

        let black_skip = Self::black_solve(black_i + 1, black_rem, black_evs, black_dp);

        let mut black_low = black_i + 1;
        let mut black_high = black_evs.len();
        let black_target = black_evs[black_i][1];
        while black_low < black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if black_evs[black_mid][0] <= black_target { black_low = black_mid + 1; }
            else { black_high = black_mid; }
        }

        let black_take = black_evs[black_i][2] + Self::black_solve(black_low, black_rem - 1, black_evs, black_dp);

        black_dp[black_i][black_rem] = black_skip.max(black_take);
        black_dp[black_i][black_rem]
    }
}
```