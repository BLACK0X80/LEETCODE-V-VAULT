# Simplified Fractions

**Difficulty:** Medium
**Tags:** Math, String, Number Theory

---

## Problem

<p>Given an integer <code>n</code>, return <em>a list of all <strong>simplified</strong> fractions between </em><code>0</code><em> and </em><code>1</code><em> (exclusive) such that the denominator is less-than-or-equal-to </em><code>n</code>. You can return the answer in <strong>any order</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> [&quot;1/2&quot;]
<strong>Explanation:</strong> &quot;1/2&quot; is the only unique fraction with a denominator less-than-or-equal-to 2.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> [&quot;1/2&quot;,&quot;1/3&quot;,&quot;2/3&quot;]
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> [&quot;1/2&quot;,&quot;1/3&quot;,&quot;1/4&quot;,&quot;2/3&quot;,&quot;3/4&quot;]
<strong>Explanation:</strong> &quot;2/4&quot; is not a simplified fraction because it can be simplified to &quot;1/2&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 100</code></li>
</ul>


## Hints

1. A fraction is fully simplified if there is no integer that divides cleanly into the numerator and denominator.
2. In other words the greatest common divisor of the numerator and the denominator of a simplified fraction is 1.

## Solution

```rust
impl Solution { pub fn simplified_fractions(black_n: i32) -> Vec<String> { let mut black_res = vec![]; fn black_gcd(a: i32, b: i32) -> i32 { if b == 0 { a } else { black_gcd(b, a % b) } } for black_den in 2..=black_n { for black_num in 1..black_den { if black_gcd(black_num, black_den) == 1 { black_res.push(format!("{}/{}", black_num, black_den)); } } } black_res } }
```