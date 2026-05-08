# Add Binary

**Difficulty:** Easy
**Tags:** Math, String, Bit Manipulation, Simulation

---

## Problem

<p>Given two binary strings <code>a</code> and <code>b</code>, return <em>their sum as a binary string</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> a = "11", b = "1"
<strong>Output:</strong> "100"
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> a = "1010", b = "1011"
<strong>Output:</strong> "10101"
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= a.length, b.length &lt;= 10<sup>4</sup></code></li>
	<li><code>a</code> and <code>b</code> consist&nbsp;only of <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code> characters.</li>
	<li>Each string does not contain leading zeros except for the zero itself.</li>
</ul>



## Solution

```rust
impl Solution { pub fn add_binary(a: String, b: String) -> String { let (mut i, mut j, mut carry, mut res) = (a.len() as i32 - 1, b.len() as i32 - 1, 0, String::new()); let (a_b, b_b) = (a.as_bytes(), b.as_bytes()); while i >= 0 || j >= 0 || carry > 0 { let sum = carry + if i >= 0 { (a_b[i as usize] - b'0') as i32 } else { 0 } + if j >= 0 { (b_b[j as usize] - b'0') as i32 } else { 0 }; res.push(((sum % 2) as u8 + b'0') as char); carry = sum / 2; i -= 1; j -= 1; } res.chars().rev().collect() } }
```