# String Without AAA or BBB

**Difficulty:** Medium
**Tags:** String, Greedy

---

## Problem

<p>Given two integers <code>a</code> and <code>b</code>, return <strong>any</strong> string <code>s</code> such that:</p>

<ul>
	<li><code>s</code> has length <code>a + b</code> and contains exactly <code>a</code> <code>&#39;a&#39;</code> letters, and exactly <code>b</code> <code>&#39;b&#39;</code> letters,</li>
	<li>The substring <code>&#39;aaa&#39;</code> does not occur in <code>s</code>, and</li>
	<li>The substring <code>&#39;bbb&#39;</code> does not occur in <code>s</code>.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> a = 1, b = 2
<strong>Output:</strong> &quot;abb&quot;
<strong>Explanation:</strong> &quot;abb&quot;, &quot;bab&quot; and &quot;bba&quot; are all correct answers.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> a = 4, b = 1
<strong>Output:</strong> &quot;aabaa&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= a, b &lt;= 100</code></li>
	<li>It is guaranteed such an <code>s</code> exists for the given <code>a</code> and <code>b</code>.</li>
</ul>



## Solution

```rust
impl Solution { pub fn str_without3a3b(mut black_a: i32, mut black_b: i32) -> String { let mut black_res = String::new(); while black_a > 0 || black_b > 0 { let black_write_a = if black_res.len() >= 2 && &black_res[black_res.len()-2..] == "aa" { false } else if black_res.len() >= 2 && &black_res[black_res.len()-2..] == "bb" { true } else { black_a >= black_b }; if black_write_a { black_a -= 1; black_res.push('a'); } else { black_b -= 1; black_res.push('b'); } } black_res } }
```