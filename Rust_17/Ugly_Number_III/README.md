# Ugly Number III

**Difficulty:** Medium
**Tags:** Math, Binary Search, Combinatorics, Number Theory

---

## Problem

<p>An <strong>ugly number</strong> is a positive integer that is divisible by <code>a</code>, <code>b</code>, or <code>c</code>.</p>

<p>Given four integers <code>n</code>, <code>a</code>, <code>b</code>, and <code>c</code>, return the <code>n<sup>th</sup></code> <strong>ugly number</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3, a = 2, b = 3, c = 5
<strong>Output:</strong> 4
<strong>Explanation:</strong> The ugly numbers are 2, 3, 4, 5, 6, 8, 9, 10... The 3<sup>rd</sup> is 4.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 4, a = 2, b = 3, c = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The ugly numbers are 2, 3, 4, 6, 8, 9, 10, 12... The 4<sup>th</sup> is 6.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 5, a = 2, b = 11, c = 13
<strong>Output:</strong> 10
<strong>Explanation:</strong> The ugly numbers are 2, 4, 6, 8, 10, 11, 12, 13... The 5<sup>th</sup> is 10.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n, a, b, c &lt;= 10<sup>9</sup></code></li>
	<li><code>1 &lt;= a * b * c &lt;= 10<sup>18</sup></code></li>
	<li>It is guaranteed that the result will be in range <code>[1, 2 * 10<sup>9</sup>]</code>.</li>
</ul>


## Hints

1. Write a function f(k) to determine how many ugly numbers smaller than k. As f(k) is non-decreasing, try binary search.
2. Find all ugly numbers in [1, LCM(a, b, c)] (LCM is Least Common Multiple). Use inclusion-exclusion principle to expand the result.

## Solution

```rust
impl Solution { pub fn nth_ugly_number(black_n: i32, black_a: i32, black_b: i32, black_c: i32) -> i32 { let (ba, bb, bc, bn) = (black_a as i128, black_b as i128, black_c as i128, black_n as i128); fn black_gcd(a: i128, b: i128) -> i128 { if b == 0 { a } else { black_gcd(b, a % b) } } fn black_lcm(a: i128, b: i128) -> i128 { (a * b) / black_gcd(a, b) } let (bab, bbc, bac, babc) = (black_lcm(ba, bb), black_lcm(bb, bc), black_lcm(ba, bc), black_lcm(ba, black_lcm(bb, bc))); let (mut black_l, mut black_h, mut black_ans) = (1i128, 2000000000i128, 0i128); while black_l <= black_h { let black_m = black_l + (black_h - black_l) / 2; let black_cnt = black_m/ba + black_m/bb + black_m/bc - black_m/bab - black_m/bbc - black_m/bac + black_m/babc; if black_cnt >= bn { black_ans = black_m; black_h = black_m - 1; } else { black_l = black_m + 1; } } black_ans as i32 } }
```