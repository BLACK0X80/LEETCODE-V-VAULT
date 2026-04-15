# Additive Number

**Difficulty:** Medium
**Tags:** String, Backtracking

---

## Problem

<p>An <strong>additive number</strong> is a string whose digits can form an <strong>additive sequence</strong>.</p>

<p>A valid <strong>additive sequence</strong> should contain <strong>at least</strong> three numbers. Except for the first two numbers, each subsequent number in the sequence must be the sum of the preceding two.</p>

<p>Given a string containing only digits, return <code>true</code> if it is an <strong>additive number</strong> or <code>false</code> otherwise.</p>

<p><strong>Note:</strong> Numbers in the additive sequence <strong>cannot</strong> have leading zeros, so sequence <code>1, 2, 03</code> or <code>1, 02, 3</code> is invalid.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> &quot;112358&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> 
The digits can form an additive sequence: 1, 1, 2, 3, 5, 8. 
1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> &quot;199100199&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> 
The additive sequence is: 1, 99, 100, 199.&nbsp;
1 + 99 = 100, 99 + 100 = 199
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 35</code></li>
	<li><code>num</code> consists only of digits.</li>
</ul>

<p>&nbsp;</p>
<p><strong>Follow up:</strong> How would you handle overflow for very large input integers?</p>



## Solution

```rust
impl Solution { pub fn is_additive_number(s: String) -> bool { let n = s.len(); for i in 1..n { for j in i+1..n { if Self::black_c(0, i, j, &s) { return true; } } } false } fn black_c(i: usize, j: usize, k: usize, s: &str) -> bool { if (s.as_bytes()[i] == b'0' && j - i > 1) || (s.as_bytes()[j] == b'0' && k - j > 1) { return false; } let (n1, n2) = (s[i..j].parse::<u128>().unwrap_or(0), s[j..k].parse::<u128>().unwrap_or(0)); let sum = (n1 + n2).to_string(); if !s[k..].starts_with(&sum) { return false; } if k + sum.len() == s.len() { return true; } Self::black_c(j, k, k + sum.len(), s) } }
```