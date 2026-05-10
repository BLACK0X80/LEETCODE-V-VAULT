# Minimum Number of Taps to Open to Water a Garden

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Greedy

---

## Problem

<p>There is a one-dimensional garden on the x-axis. The garden starts at the point <code>0</code> and ends at the point <code>n</code>. (i.e., the&nbsp;length of the garden is <code>n</code>).</p>

<p>There are <code>n + 1</code> taps located at points <code>[0, 1, ..., n]</code> in the garden.</p>

<p>Given an integer <code>n</code> and an integer array <code>ranges</code> of length <code>n + 1</code> where <code>ranges[i]</code> (0-indexed) means the <code>i-th</code> tap can water the area <code>[i - ranges[i], i + ranges[i]]</code> if it was open.</p>

<p>Return <em>the minimum number of taps</em> that should be open to water the whole garden, If the garden cannot be watered return <strong>-1</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/01/16/1685_example_1.png" style="width: 525px; height: 255px;" />
<pre>
<strong>Input:</strong> n = 5, ranges = [3,4,1,1,0,0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The tap at point 0 can cover the interval [-3,3]
The tap at point 1 can cover the interval [-3,5]
The tap at point 2 can cover the interval [1,3]
The tap at point 3 can cover the interval [2,4]
The tap at point 4 can cover the interval [4,4]
The tap at point 5 can cover the interval [5,5]
Opening Only the second tap will water the whole garden [0,5]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3, ranges = [0,0,0,0]
<strong>Output:</strong> -1
<strong>Explanation:</strong> Even if you activate all the four taps you cannot water the whole garden.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>4</sup></code></li>
	<li><code>ranges.length == n + 1</code></li>
	<li><code>0 &lt;= ranges[i] &lt;= 100</code></li>
</ul>


## Hints

1. Create intervals of the area covered by each tap, sort intervals by the left end.
2. We need to cover the interval [0, n]. we can start with the first interval and out of all intervals that intersect with it we choose the one that covers the farthest point to the right.
3. What if there is a gap between intervals that is not covered ? we should stop and return -1 as there is some interval that cannot be covered.

## Solution

```rust
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let black_n = n as usize;
        let mut black_max_reach = vec![0; black_n + 1];
        for (black_i, &black_r) in ranges.iter().enumerate() {
            let black_left = (black_i as i32 - black_r).max(0) as usize;
            let black_right = (black_i as i32 + black_r).min(n) as usize;
            black_max_reach[black_left] = black_max_reach[black_left].max(black_right);
        }
        let mut black_taps = 0;
        let mut black_curr_end = 0;
        let mut black_next_end = 0;
        for black_i in 0..black_n {
            black_next_end = black_next_end.max(black_max_reach[black_i]);
            if black_i == black_curr_end {
                black_taps += 1;
                black_curr_end = black_next_end;
                if black_curr_end >= black_n {
                    break;
                }
            }
            if black_i >= black_next_end {
                return -1;
            }
        }
        if black_curr_end >= black_n { black_taps } else { -1 }
    }
}
```