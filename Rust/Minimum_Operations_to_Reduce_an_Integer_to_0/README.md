# Minimum Operations to Reduce an Integer to 0

**Difficulty:** Medium
**Tags:** Dynamic Programming, Greedy, Bit Manipulation

---

## Problem

<p>You are given a positive integer <code>n</code>, you can do the following operation <strong>any</strong> number of times:</p>

<ul>
	<li>Add or subtract a <strong>power</strong> of <code>2</code> from <code>n</code>.</li>
</ul>

<p>Return <em>the <strong>minimum</strong> number of operations to make </em><code>n</code><em> equal to </em><code>0</code>.</p>

<p>A number <code>x</code> is power of <code>2</code> if <code>x == 2<sup>i</sup></code>&nbsp;where <code>i &gt;= 0</code><em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 39
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can do the following operations:
- Add 2<sup>0</sup> = 1 to n, so now n = 40.
- Subtract 2<sup>3</sup> = 8 from n, so now n = 32.
- Subtract 2<sup>5</sup> = 32 from n, so now n = 0.
It can be shown that 3 is the minimum number of operations we need to make n equal to 0.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 54
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can do the following operations:
- Add 2<sup>1</sup> = 2 to n, so now n = 56.
- Add 2<sup>3</sup> = 8 to n, so now n = 64.
- Subtract 2<sup>6</sup> = 64 from n, so now n = 0.
So the minimum number of operations is 3.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Can we set/unset the bits in binary representation?
2. If there are multiple adjacent ones, how can we optimally add and subtract in 2 operations such that all ones get unset?
3. Bonus: Try to solve the problem with higher constraints: n ≤ 10^18.

## Solution

```rust

```