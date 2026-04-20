# Minimum Time Difference

**Difficulty:** Medium
**Tags:** Array, Math, String, Sorting

---

## Problem

Given a list of 24-hour clock time points in <strong>&quot;HH:MM&quot;</strong> format, return <em>the minimum <b>minutes</b> difference between any two time-points in the list</em>.
<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> timePoints = ["23:59","00:00"]
<strong>Output:</strong> 1
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> timePoints = ["00:00","23:59","00:00"]
<strong>Output:</strong> 0
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= timePoints.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>timePoints[i]</code> is in the format <strong>&quot;HH:MM&quot;</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn find_min_difference(black_tp: Vec<String>) -> i32 { let mut black_m: Vec<i32> = black_tp.into_iter().map(|s| { let parts: Vec<i32> = s.split(':').map(|p| p.parse().unwrap()).collect(); parts[0] * 60 + parts[1] }).collect(); black_m.sort_unstable(); let mut black_res = 1440 + black_m[0] - black_m.last().unwrap(); for i in 1..black_m.len() { black_res = black_res.min(black_m[i] - black_m[i-1]); } black_res } }
```