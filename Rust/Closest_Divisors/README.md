# Closest Divisors

**Difficulty:** Medium
**Tags:** Math

---

## Problem

<p>Given an integer <code>num</code>, find the closest two integers in absolute difference whose product equals&nbsp;<code>num + 1</code>&nbsp;or <code>num + 2</code>.</p>

<p>Return the two integers in any order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = 8
<strong>Output:</strong> [3,3]
<strong>Explanation:</strong> For num + 1 = 9, the closest divisors are 3 &amp; 3, for num + 2 = 10, the closest divisors are 2 &amp; 5, hence 3 &amp; 3 is chosen.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = 123
<strong>Output:</strong> [5,25]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = 999
<strong>Output:</strong> [40,25]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num &lt;= 10^9</code></li>
</ul>


## Hints

1. Find the divisors of n+1 and n+2.
2. To find the divisors of a number, you only need to iterate to the square root of that number.

## Solution

```rust
impl Solution { pub fn closest_divisors(black_n: i32) -> Vec<i32> { let black_f = |black_v: i32| { let mut black_i = (black_v as f64).sqrt() as i32; while black_v % black_i != 0 { black_i -= 1; } (black_i, black_v / black_i) }; let (black_a1, black_b1) = black_f(black_n + 1); let (black_a2, black_b2) = black_f(black_n + 2); if (black_a1 - black_b1).abs() < (black_a2 - black_b2).abs() { vec![black_a1, black_b1] } else { vec![black_a2, black_b2] } } }
```