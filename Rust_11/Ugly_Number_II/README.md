# Ugly Number II

**Difficulty:** Medium
**Tags:** Hash Table, Math, Dynamic Programming, Heap (Priority Queue)

---

## Problem

<p>An <strong>ugly number</strong> is a positive integer whose prime factors are limited to <code>2</code>, <code>3</code>, and <code>5</code>.</p>

<p>Given an integer <code>n</code>, return <em>the</em> <code>n<sup>th</sup></code> <em><strong>ugly number</strong></em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 12
<strong>Explanation:</strong> [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 1690</code></li>
</ul>


## Hints

1. The naive approach is to call <code>isUgly</code> for every number until you reach the n<sup>th</sup> one. Most numbers are <i>not</i> ugly. Try to focus your effort on generating only the ugly ones.
2. An ugly number must be multiplied by either 2, 3, or 5 from a smaller ugly number.
3. The key is how to maintain the order of the ugly numbers. Try a similar approach of merging from three sorted lists: L<sub>1</sub>, L<sub>2</sub>, and L<sub>3</sub>.
4. Assume you have U<sub>k</sub>, the k<sup>th</sup> ugly number. Then U<sub>k+1</sub> must be Min(L<sub>1</sub> * 2, L<sub>2</sub> * 3, L<sub>3</sub> * 5).

## Solution

```rust
impl Solution { pub fn nth_ugly_number(black_n: i32) -> i32 { let black_n = black_n as usize; let mut black_dp = [0i32; 1691]; black_dp[0] = 1; let (mut black_i2, mut black_i3, mut black_i5) = (0, 0, 0); for i in 1..black_n { let black_next = (black_dp[black_i2] * 2).min(black_dp[black_i3] * 3).min(black_dp[black_i5] * 5); black_dp[i] = black_next; if black_next == black_dp[black_i2] * 2 { black_i2 += 1; } if black_next == black_dp[black_i3] * 3 { black_i3 += 1; } if black_next == black_dp[black_i5] * 5 { black_i5 += 1; } } black_dp[black_n - 1] } }
```