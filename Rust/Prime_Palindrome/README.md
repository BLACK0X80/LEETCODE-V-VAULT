# Prime Palindrome

**Difficulty:** Medium
**Tags:** Math, Number Theory

---

## Problem

<p>Given an integer n, return <em>the smallest <strong>prime palindrome</strong> greater than or equal to </em><code>n</code>.</p>

<p>An integer is <strong>prime</strong> if it has exactly two divisors: <code>1</code> and itself. Note that <code>1</code> is not a prime number.</p>

<ul>
	<li>For example, <code>2</code>, <code>3</code>, <code>5</code>, <code>7</code>, <code>11</code>, and <code>13</code> are all primes.</li>
</ul>

<p>An integer is a <strong>palindrome</strong> if it reads the same from left to right as it does from right to left.</p>

<ul>
	<li>For example, <code>101</code> and <code>12321</code> are palindromes.</li>
</ul>

<p>The test cases are generated so that the answer always exists and is in the range <code>[2, 2 * 10<sup>8</sup>]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> n = 6
<strong>Output:</strong> 7
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> n = 8
<strong>Output:</strong> 11
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> n = 13
<strong>Output:</strong> 101
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>8</sup></code></li>
</ul>



## Solution

```rust
impl Solution { pub fn prime_palindrome(black_n: i32) -> i32 { if 8 <= black_n && black_n <= 11 { return 11; } for black_len in 1..6 { for black_root in 10i32.pow(black_len-1)..10i32.pow(black_len) { let mut black_s = black_root.to_string(); let black_pal: i32 = (black_s.clone() + &black_s.chars().rev().skip(1).collect::<String>()).parse().unwrap(); if black_pal >= black_n && (2..=(black_pal as f64).sqrt() as i32).all(|d| black_pal % d != 0) && black_pal > 1 { return black_pal; } } } 0 } }
```