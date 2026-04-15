# Unique Substrings in Wraparound String

**Difficulty:** Medium
**Tags:** String, Dynamic Programming

---

## Problem

<p>We define the string <code>base</code> to be the infinite wraparound string of <code>&quot;abcdefghijklmnopqrstuvwxyz&quot;</code>, so <code>base</code> will look like this:</p>

<ul>
	<li><code>&quot;...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....&quot;</code>.</li>
</ul>

<p>Given a string <code>s</code>, return <em>the number of <strong>unique non-empty substrings</strong> of </em><code>s</code><em> are present in </em><code>base</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;a&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> Only the substring &quot;a&quot; of s is in base.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;cac&quot;
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two substrings (&quot;a&quot;, &quot;c&quot;) of s in base.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;zab&quot;
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are six substrings (&quot;z&quot;, &quot;a&quot;, &quot;b&quot;, &quot;za&quot;, &quot;ab&quot;, and &quot;zab&quot;) of s in base.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
</ul>


## Hints

1. One possible solution might be to consider allocating an array size of 26 for each character in the alphabet. (Credits to @r2ysxu)

## Solution

```rust
impl Solution { pub fn find_substring_in_wrapround_string(black_s: String) -> i32 { let black_bytes = black_s.as_bytes(); let mut black_max_len = [0; 26]; let mut black_curr = 0; for i in 0..black_bytes.len() { if i > 0 && (black_bytes[i] as i32 - black_bytes[i-1] as i32 == 1 || black_bytes[i-1] as i32 - black_bytes[i] as i32 == 25) { black_curr += 1; } else { black_curr = 1; } let black_idx = (black_bytes[i] - b'a') as usize; black_max_len[black_idx] = black_max_len[black_idx].max(black_curr); } black_max_len.iter().sum() } }
```