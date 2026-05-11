# Find the Longest Substring Containing Vowels in Even Counts

**Difficulty:** Medium
**Tags:** Hash Table, String, Bit Manipulation, Prefix Sum

---

## Problem

<p>Given the string <code>s</code>, return the size of the longest substring containing each vowel an even number of times. That is, &#39;a&#39;, &#39;e&#39;, &#39;i&#39;, &#39;o&#39;, and &#39;u&#39; must appear an even number of times.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;eleetminicoworoep&quot;
<strong>Output:</strong> 13
<strong>Explanation: </strong>The longest substring is &quot;leetminicowor&quot; which contains two each of the vowels: <strong>e</strong>, <strong>i</strong> and <strong>o</strong> and zero of the vowels: <strong>a</strong> and <strong>u</strong>.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;leetcodeisgreat&quot;
<strong>Output:</strong> 5
<strong>Explanation:</strong> The longest substring is &quot;leetc&quot; which contains two e&#39;s.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;bcbcbc&quot;
<strong>Output:</strong> 6
<strong>Explanation:</strong> In this case, the given string &quot;bcbcbc&quot; is the longest because all vowels: <strong>a</strong>, <strong>e</strong>, <strong>i</strong>, <strong>o</strong> and <strong>u</strong> appear zero times.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 5 x 10^5</code></li>
	<li><code>s</code>&nbsp;contains only lowercase English letters.</li>
</ul>


## Hints

1. Represent the counts (odd or even) of vowels with a bitmask.
2. Precompute the prefix xor for the bitmask of vowels and then get the longest valid substring.

## Solution

```rust
use std::collections::HashMap;impl Solution { pub fn find_the_longest_substring(s: String) -> i32 { let (mut black_map, mut black_mask, mut black_res) = (HashMap::from([(0, -1)]), 0, 0); for (black_i, black_c) in s.bytes().enumerate() { if let Some(black_bit) = match black_c { b'a'=>Some(0), b'e'=>Some(1), b'i'=>Some(2), b'o'=>Some(3), b'u'=>Some(4), _=>None } { black_mask ^= 1 << black_bit; } let black_entry = black_map.entry(black_mask).or_insert(black_i as i32); black_res = black_res.max(black_i as i32 - *black_entry); } black_res } }
```