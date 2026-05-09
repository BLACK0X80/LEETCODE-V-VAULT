# Check if Number is a Sum of Powers of Three

**Difficulty:** Medium
**Tags:** Math

---

## Problem

<p>Given an integer <code>n</code>, return <code>true</code> <em>if it is possible to represent </em><code>n</code><em> as the sum of distinct powers of three.</em> Otherwise, return <code>false</code>.</p>

<p>An integer <code>y</code> is a power of three if there exists an integer <code>x</code> such that <code>y == 3<sup>x</sup></code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 12
<strong>Output:</strong> true
<strong>Explanation:</strong> 12 = 3<sup>1</sup> + 3<sup>2</sup>
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 91
<strong>Output:</strong> true
<strong>Explanation:</strong> 91 = 3<sup>0</sup> + 3<sup>2</sup> + 3<sup>4</sup>
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 21
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. Let's note that the maximum power of 3 you'll use in your soln is 3^16
2. The number can not be represented as a sum of powers of 3 if it's ternary presentation has a 2 in it

## Solution

```rust
impl Solution { pub fn check_powers_of_three(mut n: i32) -> bool { while n > 0 { if n % 3 == 2 { return false; } n /= 3; } true } }
```