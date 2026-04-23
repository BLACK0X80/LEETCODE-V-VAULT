# Greatest Common Divisor of Strings

**Difficulty:** Easy
**Tags:** Math, String

---

## Problem

<p>For two strings <code>s</code> and <code>t</code>, we say &quot;<code>t</code> divides <code>s</code>&quot; if and only if <code>s = t + t + t + ... + t + t</code> (i.e., <code>t</code> is concatenated with itself one or more times).</p>

<p>Given two strings <code>str1</code> and <code>str2</code>, return <em>the largest string </em><code>x</code><em> such that </em><code>x</code><em> divides both </em><code>str1</code><em> and </em><code>str2</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">str1 = &quot;ABCABC&quot;, str2 = &quot;ABC&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;ABC&quot;</span></p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">str1 = &quot;ABABAB&quot;, str2 = &quot;ABAB&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;AB&quot;</span></p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">str1 = &quot;LEET&quot;, str2 = &quot;CODE&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;&quot;</span></p>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">str1 = &quot;AAAAAB&quot;, str2 = &quot;AAA&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;&quot;</span>​​​​​​​</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= str1.length, str2.length &lt;= 1000</code></li>
	<li><code>str1</code> and <code>str2</code> consist of English uppercase letters.</li>
</ul>


## Hints

1. The greatest common divisor must be a prefix of each string, so we can try all prefixes.

## Solution

```rust
impl Solution { pub fn gcd_of_strings(black_s1: String, black_s2: String) -> String { if format!("{}{}", black_s1, black_s2) != format!("{}{}", black_s2, black_s1) { return "".into(); } fn black_gcd(a: usize, b: usize) -> usize { if b == 0 { a } else { black_gcd(b, a % b) } } black_s1[..black_gcd(black_s1.len(), black_s2.len())].to_string() } }
```