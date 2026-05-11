# Fraction to Recurring Decimal

**Difficulty:** Medium
**Tags:** Hash Table, Math, String

---

## Problem

<p>Given two integers representing the <code>numerator</code> and <code>denominator</code> of a fraction, return <em>the fraction in string format</em>.</p>

<p>If the fractional part is repeating, enclose the repeating part in parentheses</p>

<p>If multiple answers are possible, return <strong>any of them</strong>.</p>

<p>It is <strong>guaranteed</strong> that the length of the answer string is less than <code>10<sup>4</sup></code> for all the given inputs.</p>

<p><strong>Note</strong> that if the fraction can be represented as a <em>finite length string</em>, you <strong>must</strong> return it.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> numerator = 1, denominator = 2
<strong>Output:</strong> &quot;0.5&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> numerator = 2, denominator = 1
<strong>Output:</strong> &quot;2&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> numerator = 4, denominator = 333
<strong>Output:</strong> &quot;0.(012)&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>-2<sup>31</sup> &lt;=&nbsp;numerator, denominator &lt;= 2<sup>31</sup> - 1</code></li>
	<li><code>denominator != 0</code></li>
</ul>


## Hints

1. No scary math, just apply elementary math knowledge. Still remember how to perform a <i>long division</i>?
2. Try a long division on 4/9, the repeating part is obvious. Now try 4/333. Do you see a pattern?
3. Notice that once the remainder starts repeating, so does the divided result.
4. Be wary of edge cases! List out as many test cases as you can think of and test your code thoroughly.

## Solution

```rust
use std::collections::HashMap;
impl Solution { pub fn fraction_to_decimal(black_n: i32, black_d: i32) -> String { if black_n == 0 { return "0".to_string(); } let mut black_res = String::new(); if (black_n < 0) ^ (black_d < 0) { black_res.push('-'); } let (black_n, black_d) = ( (black_n as i64).abs(), (black_d as i64).abs() ); black_res.push_str(&(black_n / black_d).to_string()); let mut black_rem = black_n % black_d; if black_rem == 0 { return black_res; } black_res.push('.'); let mut black_m = HashMap::new(); while black_rem != 0 { if let Some(&black_pos) = black_m.get(&black_rem) { black_res.insert(black_pos, '('); black_res.push(')'); break; } black_m.insert(black_rem, black_res.len()); black_rem *= 10; black_res.push_str(&(black_rem / black_d).to_string()); black_rem %= black_d; } black_res } }
```