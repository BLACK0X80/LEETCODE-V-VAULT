# Angle Between Hands of a Clock

**Difficulty:** Medium
**Tags:** Math

---

## Problem

<p>Given two numbers, <code>hour</code> and <code>minutes</code>, return <em>the smaller angle (in degrees) formed between the </em><code>hour</code><em> and the </em><code>minute</code><em> hand</em>.</p>

<p>Answers within <code>10<sup>-5</sup></code> of the actual value will be accepted as correct.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_1_1673.png" style="width: 300px; height: 296px;" />
<pre>
<strong>Input:</strong> hour = 12, minutes = 30
<strong>Output:</strong> 165
</pre>

<p><strong class="example">Example 2:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_2_1673.png" style="width: 300px; height: 301px;" />
<pre>
<strong>Input:</strong> hour = 3, minutes = 30
<strong>Output:</strong> 75
</pre>

<p><strong class="example">Example 3:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_3_1673.png" style="width: 300px; height: 301px;" />
<pre>
<strong>Input:</strong> hour = 3, minutes = 15
<strong>Output:</strong> 7.5
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= hour &lt;= 12</code></li>
	<li><code>0 &lt;= minutes &lt;= 59</code></li>
</ul>


## Hints

1. The tricky part is determining how the minute hand affects the position of the hour hand.
2. Calculate the angles separately then find the difference.

## Solution

```rust
impl Solution { pub fn angle_clock(black_h: i32, black_m: i32) -> f64 { let black_ha = (black_h % 12) as f64 * 30.0 + black_m as f64 * 0.5; let black_ma = black_m as f64 * 6.0; let black_diff = (black_ha - black_ma).abs(); black_diff.min(360.0 - black_diff) } }
```