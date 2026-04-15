# Super Ugly Number

**Difficulty:** Medium
**Tags:** Array, Math, Dynamic Programming

---

## Problem

<p>A <strong>super ugly number</strong> is a positive integer whose prime factors are in the array <code>primes</code>.</p>

<p>Given an integer <code>n</code> and an array of integers <code>primes</code>, return <em>the</em> <code>n<sup>th</sup></code> <em><strong>super ugly number</strong></em>.</p>

<p>The <code>n<sup>th</sup></code> <strong>super ugly number</strong> is <strong>guaranteed</strong> to fit in a <strong>32-bit</strong> signed integer.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 12, primes = [2,7,13,19]
<strong>Output:</strong> 32
<strong>Explanation:</strong> [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12 super ugly numbers given primes = [2,7,13,19].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1, primes = [2,3,5]
<strong>Output:</strong> 1
<strong>Explanation:</strong> 1 has no prime factors, therefore all of its prime factors are in the array primes = [2,3,5].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= primes.length &lt;= 100</code></li>
	<li><code>2 &lt;= primes[i] &lt;= 1000</code></li>
	<li><code>primes[i]</code> is <strong>guaranteed</strong> to be a prime number.</li>
	<li>All the values of <code>primes</code> are <strong>unique</strong> and sorted in <strong>ascending order</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn nth_super_ugly_number(black_n: i32, black_p: Vec<i32>) -> i32 { let black_n = black_n as usize; let mut black_dp = vec![1i32; black_n]; let mut black_idx = vec![0usize; black_p.len()]; for i in 1..black_n { let mut black_next = i32::MAX; for j in 0..black_p.len() { let black_v = (black_dp[black_idx[j]] as u64) * (black_p[j] as u64); if black_v < black_next as u64 { black_next = black_v as i32; } } black_dp[i] = black_next; for j in 0..black_p.len() { if black_next as u64 == (black_dp[black_idx[j]] as u64) * (black_p[j] as u64) { black_idx[j] += 1; } } } black_dp[black_n - 1] } }
```