# Multiply Strings

**Difficulty:** Medium
**Tags:** Math, String, Simulation

---

## Problem

<p>Given two non-negative integers <code>num1</code> and <code>num2</code> represented as strings, return the product of <code>num1</code> and <code>num2</code>, also represented as a string.</p>

<p><strong>Note:</strong>&nbsp;You must not use any built-in BigInteger library or convert the inputs to integer directly.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> num1 = "2", num2 = "3"
<strong>Output:</strong> "6"
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> num1 = "123", num2 = "456"
<strong>Output:</strong> "56088"
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num1.length, num2.length &lt;= 200</code></li>
	<li><code>num1</code> and <code>num2</code> consist of digits only.</li>
	<li>Both <code>num1</code> and <code>num2</code>&nbsp;do not contain any leading zero, except the number <code>0</code> itself.</li>
</ul>



## Solution

```rust
impl Solution { pub fn multiply(b1: String, b2: String) -> String {
    if b1 == "0" || b2 == "0" { return "0".into() } let mut r = vec![0; b1.len() + b2.len()];
    b1.bytes().rev().enumerate().for_each(|(i, x)| b2.bytes().rev().enumerate().for_each(|(j, y)| { let p = (x - 48) as i32 * (y - 48) as i32 + r[i+j]; r[i+j] = p % 10; r[i+j+1] += p / 10; }));
    while r.len() > 1 && r.last() == Some(&0) { r.pop(); } r.iter().rev().map(|&x| (x + 48) as u8 as char).collect()
}}
```