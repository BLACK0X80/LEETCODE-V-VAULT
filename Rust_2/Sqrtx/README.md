# Sqrt(x)

**Difficulty:** Easy
**Tags:** Math, Binary Search

---

## Problem

<p>Given a non-negative integer <code>x</code>, return <em>the square root of </em><code>x</code><em> rounded down to the nearest integer</em>. The returned integer should be <strong>non-negative</strong> as well.</p>

<p>You <strong>must not use</strong> any built-in exponent function or operator.</p>

<ul>
	<li>For example, do not use <code>pow(x, 0.5)</code> in c++ or <code>x ** 0.5</code> in python.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> x = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> The square root of 4 is 2, so we return 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> x = 8
<strong>Output:</strong> 2
<strong>Explanation:</strong> The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= x &lt;= 2<sup>31</sup> - 1</code></li>
</ul>


## Hints

1. Try exploring all integers. (Credits: @annujoshi)
2. Use the sorted property of integers to reduced the search space. (Credits: @annujoshi)

## Solution

```rust
impl Solution { pub fn my_sqrt(x: i32) -> i32 { if x < 2 { return x; } let (mut l, mut r, x_64) = (1, x / 2, x as i64); while l <= r { let m = l + (r - l) / 2; if m as i64 * m as i64 <= x_64 { l = m + 1; } else { r = m - 1; } } r } }
```