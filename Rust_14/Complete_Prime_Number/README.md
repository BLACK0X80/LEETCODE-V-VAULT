# Complete Prime Number

**Difficulty:** Medium
**Tags:** Math, Enumeration, Number Theory

---

## Problem

<p>You are given an integer <code>num</code>.</p>

<p>A number <code>num</code> is called a <strong>Complete <span data-keyword="prime-number">Prime Number</span></strong> if every <strong>prefix</strong> and every <strong>suffix</strong> of <code>num</code> is <strong>prime</strong>.</p>

<p>Return <code>true</code> if <code>num</code> is a Complete Prime Number, otherwise return <code>false</code>.</p>

<p><strong>Note</strong>:</p>

<ul>
	<li>A <strong>prefix</strong> of a number is formed by the <strong>first</strong> <code>k</code> digits of the number.</li>
	<li>A <strong>suffix</strong> of a number is formed by the <strong>last</strong> <code>k</code> digits of the number.</li>
	<li>Single-digit numbers are considered Complete Prime Numbers only if they are <strong>prime</strong>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num = 23</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li><strong>​​​​​​​</strong>Prefixes of <code>num = 23</code> are 2 and 23, both are prime.</li>
	<li>Suffixes of <code>num = 23</code> are 3 and 23, both are prime.</li>
	<li>All prefixes and suffixes are prime, so 23 is a Complete Prime Number and the answer is <code>true</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num = 39</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Prefixes of <code>num = 39</code> are 3 and 39. 3 is prime, but 39 is not prime.</li>
	<li>Suffixes of <code>num = 39</code> are 9 and 39. Both 9 and 39 are not prime.</li>
	<li>At least one prefix or suffix is not prime, so 39 is not a Complete Prime Number and the answer is <code>false</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">num = 7</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>7 is prime, so all its prefixes and suffixes are prime and the answer is <code>true</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Check primality for all prefixes and all suffixes of <code>num</code> and return true only if every one is prime.

## Solution

```rust
impl Solution { pub fn complete_prime(black_num: i32) -> bool { let black_is_p = |mut black_x: i32| { if black_x < 2 { return false; } for black_i in 2..=(black_x as f64).sqrt() as i32 { if black_x % black_i == 0 { return false; } } true }; let (black_s, mut black_ok) = (black_num.to_string(), true); for black_i in 1..=black_s.len() { if !black_is_p(black_s[..black_i].parse().unwrap()) || !black_is_p(black_s[black_s.len()-black_i..].parse().unwrap()) { black_ok = false; break; } } black_ok } }
```