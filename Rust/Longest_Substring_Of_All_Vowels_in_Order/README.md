# Longest Substring Of All Vowels in Order

**Difficulty:** Medium
**Tags:** String, Sliding Window

---

## Problem

<p>A string is considered <strong>beautiful</strong> if it satisfies the following conditions:</p>

<ul>
	<li>Each of the 5 English vowels (<code>&#39;a&#39;</code>, <code>&#39;e&#39;</code>, <code>&#39;i&#39;</code>, <code>&#39;o&#39;</code>, <code>&#39;u&#39;</code>) must appear <strong>at least once</strong> in it.</li>
	<li>The letters must be sorted in <strong>alphabetical order</strong> (i.e. all <code>&#39;a&#39;</code>s before <code>&#39;e&#39;</code>s, all <code>&#39;e&#39;</code>s before <code>&#39;i&#39;</code>s, etc.).</li>
</ul>

<p>For example, strings <code>&quot;aeiou&quot;</code> and <code>&quot;aaaaaaeiiiioou&quot;</code> are considered <strong>beautiful</strong>, but <code>&quot;uaeio&quot;</code>, <code>&quot;aeoiu&quot;</code>, and <code>&quot;aaaeeeooo&quot;</code> are <strong>not beautiful</strong>.</p>

<p>Given a string <code>word</code> consisting of English vowels, return <em>the <strong>length of the longest beautiful substring</strong> of </em><code>word</code><em>. If no such substring exists, return </em><code>0</code>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters in a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> word = &quot;aeiaaio<u>aaaaeiiiiouuu</u>ooaauuaeiu&quot;
<strong>Output:</strong> 13
<b>Explanation:</b> The longest beautiful substring in word is &quot;aaaaeiiiiouuu&quot; of length 13.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> word = &quot;aeeeiiiioooauuu<u>aeiou</u>&quot;
<strong>Output:</strong> 5
<b>Explanation:</b> The longest beautiful substring in word is &quot;aeiou&quot; of length 5.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> word = &quot;a&quot;
<strong>Output:</strong> 0
<b>Explanation:</b> There is no beautiful substring, so return 0.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word.length &lt;= 5 * 10<sup>5</sup></code></li>
	<li><code>word</code> consists of characters <code>&#39;a&#39;</code>, <code>&#39;e&#39;</code>, <code>&#39;i&#39;</code>, <code>&#39;o&#39;</code>, and <code>&#39;u&#39;</code>.</li>
</ul>


## Hints

1. Start from each 'a' and find the longest beautiful substring starting at that index.
2. Based on the current character decide if you should include the next character in the beautiful substring.

## Solution

```rust
impl Solution { pub fn longest_beautiful_substring(black_w: String) -> i32 { let (black_b, mut black_res, mut black_cur, mut black_cnt) = (black_w.as_bytes(), 0, 1, 1); for black_i in 1..black_b.len() { if black_b[black_i] >= black_b[black_i-1] { black_cur += 1; if black_b[black_i] > black_b[black_i-1] { black_cnt += 1; } } else { black_cur = 1; black_cnt = 1; } if black_cnt == 5 { black_res = black_res.max(black_cur); } } black_res } }
```