# Remove Covered Intervals

**Difficulty:** Medium
**Tags:** Array, Sorting

---

## Problem

<p>Given an array <code>intervals</code> where <code>intervals[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> represent the interval <code>[l<sub>i</sub>, r<sub>i</sub>)</code>, remove all intervals that are covered by another interval in the list.</p>

<p>The interval <code>[a, b)</code> is covered by the interval <code>[c, d)</code> if and only if <code>c &lt;= a</code> and <code>b &lt;= d</code>.</p>

<p>Return <em>the number of remaining intervals</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> intervals = [[1,4],[3,6],[2,8]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Interval [3,6] is covered by [2,8], therefore it is removed.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> intervals = [[1,4],[2,3]]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= intervals.length &lt;= 1000</code></li>
	<li><code>intervals[i].length == 2</code></li>
	<li><code>0 &lt;= l<sub>i</sub> &lt; r<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
	<li>All the given intervals are <strong>unique</strong>.</li>
</ul>


## Hints

1. How to check if an interval is covered by another?
2. Compare each interval to all others and check if it is covered by any interval.

## Solution

```rust
impl Solution { pub fn remove_covered_intervals(mut black_i: Vec<Vec<i32>>) -> i32 { black_i.sort_unstable_by(|black_a, black_b| black_a[0].cmp(&black_b[0]).then(black_b[1].cmp(&black_a[1]))); let (mut black_res, mut black_max) = (0, 0); for black_v in black_i { if black_v[1] > black_max { black_res += 1; black_max = black_v[1]; } } black_res } }
```