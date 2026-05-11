# Find the Count of Numbers Which Are Not Special

**Difficulty:** Medium
**Tags:** Array, Math, Number Theory

---

## Problem

<p>You are given 2 <strong>positive</strong> integers <code>l</code> and <code>r</code>. For any number <code>x</code>, all positive divisors of <code>x</code> <em>except</em> <code>x</code> are called the <strong>proper divisors</strong> of <code>x</code>.</p>

<p>A number is called <strong>special</strong> if it has exactly 2 <strong>proper divisors</strong>. For example:</p>

<ul>
	<li>The number 4 is <em>special</em> because it has proper divisors 1 and 2.</li>
	<li>The number 6 is <em>not special</em> because it has proper divisors 1, 2, and 3.</li>
</ul>

<p>Return the count of numbers in the range <code>[l, r]</code> that are <strong>not</strong> <strong>special</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 5, r = 7</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>There are no special numbers in the range <code>[5, 7]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">l = 4, r = 16</span></p>

<p><strong>Output:</strong> <span class="example-io">11</span></p>

<p><strong>Explanation:</strong></p>

<p>The special numbers in the range <code>[4, 16]</code> are 4 and 9.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= l &lt;= r &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. A special number must be a square of a prime number.
2. We need to find all primes in the range <code>[sqrt(l), sqrt(r)]</code>.
3. Use sieve to find primes till <code>sqrt(10<sup>9</sup>)</code>.

## Solution

```rust
impl Solution { pub fn non_special_count(l: i32, r: i32) -> i32 { let black_limit = (r as f64).sqrt() as usize; let mut black_prime = vec![true; black_limit + 1]; let mut black_special = 0; if black_limit >= 0 { black_prime[0] = false; if black_limit >= 1 { black_prime[1] = false; } } for i in 2..=black_limit { if black_prime[i] { let black_sq = (i * i) as i32; if black_sq >= l && black_sq <= r { black_special += 1; } let mut j = i * i; while j <= black_limit { black_prime[j] = false; j += i; } } } (r - l + 1) - black_special } }
```