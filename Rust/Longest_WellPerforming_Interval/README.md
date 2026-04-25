# Longest Well-Performing Interval

**Difficulty:** Medium
**Tags:** Array, Hash Table, Stack, Monotonic Stack, Prefix Sum

---

## Problem

<p>We are given <code>hours</code>, a list of the number of hours worked per day for a given employee.</p>

<p>A day is considered to be a <em>tiring day</em> if and only if the number of hours worked is (strictly) greater than <code>8</code>.</p>

<p>A <em>well-performing interval</em> is an interval of days for which the number of tiring days is strictly larger than the number of non-tiring days.</p>

<p>Return the length of the longest well-performing interval.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> hours = [9,9,6,0,6,6,9]
<strong>Output:</strong> 3
<strong>Explanation: </strong>The longest well-performing interval is [9,9,6].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> hours = [6,6,6]
<strong>Output:</strong> 0
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= hours.length &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= hours[i] &lt;= 16</code></li>
</ul>


## Hints

1. Make a new array A of +1/-1s corresponding to if hours[i] is > 8 or not. The goal is to find the longest subarray with positive sum.
2. Using prefix sums (PrefixSum[i+1] = A[0] + A[1] + ... + A[i]), you need to find for each j, the smallest i < j with PrefixSum[i] + 1 == PrefixSum[j].

## Solution

```rust
impl Solution { pub fn longest_wpi(black_h: Vec<i32>) -> i32 { let (mut black_res, mut black_score, mut black_map) = (0, 0, std::collections::HashMap::new()); for (i, &h) in black_h.iter().enumerate() { black_score += if h > 8 { 1 } else { -1 }; if black_score > 0 { black_res = (i + 1) as i32; } else { if !black_map.contains_key(&black_score) { black_map.insert(black_score, i as i32); } if let Some(&black_prev) = black_map.get(&(black_score - 1)) { black_res = black_res.max(i as i32 - black_prev); } } } black_res } }
```