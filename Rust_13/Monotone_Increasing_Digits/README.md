# Monotone Increasing Digits

**Difficulty:** Medium
**Tags:** Math, Greedy

---

## Problem

<p>An integer has <strong>monotone increasing digits</strong> if and only if each pair of adjacent digits <code>x</code> and <code>y</code> satisfy <code>x &lt;= y</code>.</p>

<p>Given an integer <code>n</code>, return <em>the largest number that is less than or equal to </em><code>n</code><em> with <strong>monotone increasing digits</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 9
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1234
<strong>Output:</strong> 1234
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 332
<strong>Output:</strong> 299
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Build the answer digit by digit, adding the largest possible one that would make the number still less than or equal to N.

## Solution

```rust
impl Solution { pub fn monotone_increasing_digits(black_n: i32) -> i32 { let mut black_s = black_n.to_string().into_bytes(); let mut black_marker = black_s.len(); for i in (1..black_s.len()).rev() { if black_s[i-1] > black_s[i] { black_marker = i; black_s[i-1] -= 1; } } for i in black_marker..black_s.len() { black_s[i] = b'9'; } String::from_utf8(black_s).unwrap().parse().unwrap() } }
```