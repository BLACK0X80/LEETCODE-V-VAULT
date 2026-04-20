# Maximum Number of Vowels in a Substring of Given Length

**Difficulty:** Medium
**Tags:** String, Sliding Window

---

## Problem

<p>Given a string <code>s</code> and an integer <code>k</code>, return <em>the maximum number of vowel letters in any substring of </em><code>s</code><em> with length </em><code>k</code>.</p>

<p><strong>Vowel letters</strong> in English are <code>&#39;a&#39;</code>, <code>&#39;e&#39;</code>, <code>&#39;i&#39;</code>, <code>&#39;o&#39;</code>, and <code>&#39;u&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abciiidef&quot;, k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The substring &quot;iii&quot; contains 3 vowel letters.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;aeiou&quot;, k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> Any substring of length 2 contains 2 vowels.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;leetcode&quot;, k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> &quot;lee&quot;, &quot;eet&quot; and &quot;ode&quot; contain 2 vowels.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
	<li><code>1 &lt;= k &lt;= s.length</code></li>
</ul>


## Hints

1. Keep a window of size k and maintain the number of vowels in it.
2. Keep moving the window and update the number of vowels while moving. Answer is max number of vowels of any window.

## Solution

```rust
impl Solution { pub fn max_vowels(black_s: String, black_k: i32) -> i32 { let (black_b, black_v) = (black_s.as_bytes(), |black_c: u8| matches!(black_c, b'a' | b'e' | b'i' | b'o' | b'u')); let (mut black_cur, mut black_res) = (black_b.iter().take(black_k as usize).filter(|&&black_c| black_v(black_c)).count() as i32, 0); black_res = black_cur; for black_i in black_k as usize..black_b.len() { black_cur += black_v(black_b[black_i]) as i32 - black_v(black_b[black_i - black_k as usize]) as i32; black_res = black_res.max(black_cur); if black_res == black_k { return black_k; } } black_res } }
```