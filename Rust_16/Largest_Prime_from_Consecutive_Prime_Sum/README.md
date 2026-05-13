# Largest Prime from Consecutive Prime Sum

**Difficulty:** Medium
**Tags:** Array, Math, Number Theory

---

## Problem

<p>You are given an integer <code>n</code>.</p>

<p>Return the <strong>largest <span data-keyword="prime-number">prime number</span></strong> less than or equal to <code>n</code> that can be expressed as the <strong>sum</strong> of one or more <strong>consecutive prime numbers</strong> starting from 2. If no such number exists, return 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 20</span></p>

<p><strong>Output:</strong> <span class="example-io">17</span></p>

<p><strong>Explanation:</strong></p>

<p>The prime numbers less than or equal to <code>n = 20</code> which are consecutive prime sums are:</p>

<ul>
	<li>
	<p><code>2 = 2</code></p>
	</li>
	<li>
	<p><code>5 = 2 + 3</code></p>
	</li>
	<li>
	<p><code>17 = 2 + 3 + 5 + 7</code></p>
	</li>
</ul>

<p>The largest is 17, so it is the answer.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The only consecutive prime sum less than or equal to 2 is 2 itself.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 5 * 10<sup>5</sup></code></li>
</ul>


## Hints

1. Generate all prime numbers up to <code>n</code> (use sieve or trial division).
2. Compute consecutive sums starting from 2 until the sum exceeds <code>n</code>.
3. Find the largest sum that is also a prime.

## Solution

```rust
impl Solution { pub fn largest_prime(black_n: i32) -> i32 { let mut black_p = vec![true; black_n as usize + 1]; let mut black_primes = vec![]; for black_i in 2..=black_n as usize { if black_p[black_i] { black_primes.push(black_i as i32); for black_j in (black_i * black_i..=black_n as usize).step_by(black_i) { black_p[black_j] = false; } } } let (mut black_s, mut black_ans) = (0, 0); for black_p_val in black_primes { black_s += black_p_val; if black_s > black_n { break; } if black_p[black_s as usize] { black_ans = black_s; } } black_ans } }
```