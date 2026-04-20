# Replace the Substring for Balanced String

**Difficulty:** Medium
**Tags:** String, Sliding Window

---

## Problem

<p>You are given a string s of length <code>n</code> containing only four kinds of characters: <code>&#39;Q&#39;</code>, <code>&#39;W&#39;</code>, <code>&#39;E&#39;</code>, and <code>&#39;R&#39;</code>.</p>

<p>A string is said to be <strong>balanced</strong><em> </em>if each of its characters appears <code>n / 4</code> times where <code>n</code> is the length of the string.</p>

<p>Return <em>the minimum length of the substring that can be replaced with <strong>any</strong> other string of the same length to make </em><code>s</code><em> <strong>balanced</strong></em>. If s is already <strong>balanced</strong>, return <code>0</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;QWER&quot;
<strong>Output:</strong> 0
<strong>Explanation:</strong> s is already balanced.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;QQWE&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> We need to replace a &#39;Q&#39; to &#39;R&#39;, so that &quot;RQWE&quot; (or &quot;QRWE&quot;) is balanced.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;QQQW&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can replace the first &quot;QQ&quot; to &quot;ER&quot;. 
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == s.length</code></li>
	<li><code>4 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>n</code> is a multiple of <code>4</code>.</li>
	<li><code>s</code> contains only <code>&#39;Q&#39;</code>, <code>&#39;W&#39;</code>, <code>&#39;E&#39;</code>, and <code>&#39;R&#39;</code>.</li>
</ul>


## Hints

1. Use 2-pointers algorithm to make sure all amount of characters outside the 2 pointers are smaller or equal to n/4.
2. That means you need to count the amount of each letter and make sure the amount is enough.

## Solution

```rust
impl Solution { pub fn balanced_string(black_s: String) -> i32 { let (black_b, black_n) = (black_s.as_bytes(), black_s.len()); let (mut black_c, mut black_l, mut black_res, black_t) = ([0i32; 128], 0, black_n as i32, (black_n / 4) as i32); for &black_v in black_b { black_c[black_v as usize] += 1; } if [b'Q', b'W', b'E', b'R'].iter().all(|&black_k| black_c[black_k as usize] <= black_t) { return 0; } for black_r in 0..black_n { black_c[black_b[black_r] as usize] -= 1; while [b'Q', b'W', b'E', b'R'].iter().all(|&black_k| black_c[black_k as usize] <= black_t) { black_res = black_res.min((black_r - black_l + 1) as i32); black_c[black_b[black_l] as usize] += 1; black_l += 1; } } black_res } }
```