# Resulting String After Adjacent Removals

**Difficulty:** Medium
**Tags:** String, Stack, Simulation

---

## Problem

<p>You are given a string <code>s</code> consisting of lowercase English letters.</p>

<p>You <strong>must</strong> repeatedly perform the following operation while the string <code>s</code> has <strong>at least</strong> two <strong>consecutive </strong>characters:</p>

<ul>
	<li>Remove the <strong>leftmost</strong> pair of <strong>adjacent</strong> characters in the string that are <strong>consecutive</strong> in the alphabet, in either order (e.g., <code>&#39;a&#39;</code> and <code>&#39;b&#39;</code>, or <code>&#39;b&#39;</code> and <code>&#39;a&#39;</code>).</li>
	<li>Shift the remaining characters to the left to fill the gap.</li>
</ul>

<p>Return the resulting string after no more operations can be performed.</p>

<p><strong>Note:</strong> Consider the alphabet as circular, thus <code>&#39;a&#39;</code> and <code>&#39;z&#39;</code> are consecutive.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;abc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;c&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Remove <code>&quot;ab&quot;</code> from the string, leaving <code>&quot;c&quot;</code> as the remaining string.</li>
	<li>No further operations are possible. Thus, the resulting string after all possible removals is <code>&quot;c&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;adcb&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Remove <code>&quot;dc&quot;</code> from the string, leaving <code>&quot;ab&quot;</code> as the remaining string.</li>
	<li>Remove <code>&quot;ab&quot;</code> from the string, leaving <code>&quot;&quot;</code> as the remaining string.</li>
	<li>No further operations are possible. Thus, the resulting string after all possible removals is <code>&quot;&quot;</code>.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;zadb&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;db&quot;</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Remove <code>&quot;za&quot;</code> from the string, leaving <code>&quot;db&quot;</code> as the remaining string.</li>
	<li>No further operations are possible. Thus, the resulting string after all possible removals is <code>&quot;db&quot;</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
</ul>


## Hints

1. Traverse the string from left to right and use a stack to perform the removals.

## Solution

```rust
impl Solution { pub fn resulting_string(black_s: String) -> String { let mut black_st: Vec<char> = vec![]; for black_c in black_s.chars() { if let Some(&black_last) = black_st.last() { let black_diff = (black_c as i32 - black_last as i32).abs(); if black_diff == 1 || black_diff == 25 { black_st.pop(); continue; } } black_st.push(black_c); } black_st.into_iter().collect() } }
```