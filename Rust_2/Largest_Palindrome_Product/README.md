# Largest Palindrome Product

**Difficulty:** Hard
**Tags:** Math, Enumeration

---

## Problem

<p>Given an integer n, return <em>the <strong>largest palindromic integer</strong> that can be represented as the product of two <code>n</code>-digits integers</em>. Since the answer can be very large, return it <strong>modulo</strong> <code>1337</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 987
Explanation: 99 x 91 = 9009, 9009 % 1337 = 987
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 9
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 8</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 { return 9; }
        let upper = 10i64.pow(n as u32) - 1;
        let lower = 10i64.pow(n as u32 - 1);
        for a in (lower..=upper).rev() {
            let s = a.to_string();
            let rev: String = s.chars().rev().collect();
            let p: i64 = format!("{}{}", s, rev).parse().unwrap();
            let mut b = upper;
            while b * b >= p {
                if p % b == 0 { return (p % 1337) as i32; }
                b -= 1;
            }
        }
        0
    }
}
```