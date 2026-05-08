# Basic Calculator II

**Difficulty:** Medium
**Tags:** Math, String, Stack

---

## Problem

<p>Given a string <code>s</code> which represents an expression, <em>evaluate this expression and return its value</em>.&nbsp;</p>

<p>The integer division should truncate toward zero.</p>

<p>You may assume that the given expression is always valid. All intermediate results will be in the range of <code>[-2<sup>31</sup>, 2<sup>31</sup> - 1]</code>.</p>

<p><strong>Note:</strong> You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as <code>eval()</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> s = "3+2*2"
<strong>Output:</strong> 7
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> s = " 3/2 "
<strong>Output:</strong> 1
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> s = " 3+5 / 2 "
<strong>Output:</strong> 5
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 3 * 10<sup>5</sup></code></li>
	<li><code>s</code> consists of integers and operators <code>(&#39;+&#39;, &#39;-&#39;, &#39;*&#39;, &#39;/&#39;)</code> separated by some number of spaces.</li>
	<li><code>s</code> represents <strong>a valid expression</strong>.</li>
	<li>All the integers in the expression are non-negative integers in the range <code>[0, 2<sup>31</sup> - 1]</code>.</li>
	<li>The answer is <strong>guaranteed</strong> to fit in a <strong>32-bit integer</strong>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn calculate(black_s: String) -> i32 { let (mut black_res, mut black_last, mut black_curr, mut black_op, black_b) = (0, 0, 0, b'+', black_s.as_bytes()); for (i, &b) in black_b.iter().enumerate() { if b.is_ascii_digit() { black_curr = black_curr * 10 + (b - b'0') as i32; } if (!b.is_ascii_digit() && b != b' ') || i == black_b.len() - 1 { match black_op { b'+' => { black_res += black_last; black_last = black_curr; } b'-' => { black_res += black_last; black_last = -black_curr; } b'*' => black_last *= black_curr, b'/' => black_last /= black_curr, _ => {} } black_op = b; black_curr = 0; } } black_res + black_last } }
```