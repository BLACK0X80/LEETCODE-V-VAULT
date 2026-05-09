# Break a Palindrome

**Difficulty:** Medium
**Tags:** String, Greedy

---

## Problem

<p>Given a palindromic string of lowercase English letters <code>palindrome</code>, replace <strong>exactly one</strong> character with any lowercase English letter so that the resulting string is <strong>not</strong> a palindrome and that it is the <strong>lexicographically smallest</strong> one possible.</p>

<p>Return <em>the resulting string. If there is no way to replace a character to make it not a palindrome, return an <strong>empty string</strong>.</em></p>

<p>A string <code>a</code> is lexicographically smaller than a string <code>b</code> (of the same length) if in the first position where <code>a</code> and <code>b</code> differ, <code>a</code> has a character strictly smaller than the corresponding character in <code>b</code>. For example, <code>&quot;abcc&quot;</code> is lexicographically smaller than <code>&quot;abcd&quot;</code> because the first position they differ is at the fourth character, and <code>&#39;c&#39;</code> is smaller than <code>&#39;d&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> palindrome = &quot;abccba&quot;
<strong>Output:</strong> &quot;aaccba&quot;
<strong>Explanation:</strong> There are many ways to make &quot;abccba&quot; not a palindrome, such as &quot;<u>z</u>bccba&quot;, &quot;a<u>a</u>ccba&quot;, and &quot;ab<u>a</u>cba&quot;.
Of all the ways, &quot;aaccba&quot; is the lexicographically smallest.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> palindrome = &quot;a&quot;
<strong>Output:</strong> &quot;&quot;
<strong>Explanation:</strong> There is no way to replace a single character to make &quot;a&quot; not a palindrome, so return an empty string.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= palindrome.length &lt;= 1000</code></li>
	<li><code>palindrome</code> consists of only lowercase English letters.</li>
</ul>


## Hints

1. How to detect if there is impossible to perform the replacement? Only when the length = 1.
2. Change the first non 'a' character to 'a'.
3. What if the string has only 'a'?
4. Change the last character to 'b'.

## Solution

```rust
impl Solution { pub fn break_palindrome(mut black_p: String) -> String { let black_n = black_p.len(); if black_n <= 1 { return "".to_string(); } let mut black_b = black_p.into_bytes(); if let Some(black_i) = (0..black_n / 2).find(|&i| black_b[i] != b'a') { black_b[black_i] = b'a'; } else { black_b[black_n - 1] = b'b'; } String::from_utf8(black_b).unwrap() } }
```